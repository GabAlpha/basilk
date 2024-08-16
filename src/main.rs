use std::{
    error::Error,
    fmt::Debug,
    fs,
    io::{self, stdout},
};

use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode},
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
        ExecutableCommand,
    },
    prelude::*,
    widgets::*,
};
use serde_json::from_str;
use tui_input::{backend::crossterm::EventHandler, Input};

mod config;
mod project;
mod task;
mod ui;

use config::PATH_JSON;
use project::Project;
use task::{Task, TASK_STATUSES};
use ui::Ui;

#[derive(Default, PartialEq, Debug)]
pub enum ViewMode {
    #[default]
    ViewProjects,
    RenameProject,

    ViewTasks,
    RenameTask,
    ChangeStatusTask,
    AddTask,
}

pub struct App {
    selected_project_index: ListState,
    selected_task_index: ListState,
    selected_status_task_index: ListState,
    view_mode: ViewMode,
    projects: Vec<Project>,
}

fn init_terminal() -> Result<Terminal<impl Backend>, Box<dyn Error>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

fn restore_terminal() -> Result<(), Box<dyn Error>> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    let terminal = init_terminal()?;

    // create app and run it
    App::setup().run(terminal)?;

    restore_terminal()?;

    Ok(())
}

impl App {
    fn setup() -> Self {
        Self {
            selected_project_index: ListState::default().with_selected(Some(0)),
            selected_task_index: ListState::default().with_selected(Some(0)),
            selected_status_task_index: ListState::default().with_selected(Some(0)),
            view_mode: ViewMode::default(),
            projects: App::read_json(),
        }
    }

    fn read_json() -> Vec<Project> {
        let json = fs::read_to_string(PATH_JSON).unwrap();
        return from_str::<Vec<Project>>(&json).unwrap();
    }

    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        let mut input = Input::default();

        let mut items: Vec<ListItem> = vec![];
        Project::load(self, &mut items);

        let mut status_items: Vec<ListItem> = vec![];
        Task::load_statues(&mut status_items);

        loop {
            terminal.draw(|f| self.render(f, f.size(), &input, &items, &status_items))?;

            if let Event::Key(key) = event::read()? {
                use KeyCode::*;
                match self.view_mode {
                    ViewMode::ViewProjects => match key.code {
                        Enter | Right => {
                            Task::load(self, &mut items);
                            self.selected_task_index.select(Some(0));
                            self.view_mode = ViewMode::ViewTasks
                        }
                        Char('r') => {
                            let current_project =
                                &self.projects[self.selected_project_index.selected().unwrap()];
                            input = input.clone().with_value(current_project.title.clone());
                            self.view_mode = ViewMode::RenameProject
                        }
                        Char('q') => return Ok(()),
                        Down => self.next(&items),
                        Up => self.previous(&items),
                        _ => {}
                    },
                    ViewMode::RenameProject => match key.code {
                        Enter => {
                            Project::rename(self, &mut items, input.value());
                            self.view_mode = ViewMode::ViewProjects;
                            input.reset()
                        }
                        Esc => {
                            self.view_mode = ViewMode::ViewProjects;
                            input.reset()
                        }
                        Char('q') => return Ok(()),
                        _ => {
                            input.handle_event(&Event::Key(key));
                        }
                    },

                    ViewMode::ViewTasks => match key.code {
                        Esc | Left => {
                            Project::load(self, &mut items);
                            self.view_mode = ViewMode::ViewProjects;
                        }
                        Enter => {
                            let selected_task_status = &self.projects
                                [self.selected_project_index.selected().unwrap()]
                            .tasks[self.selected_task_index.selected().unwrap()]
                            .status;

                            let index = TASK_STATUSES
                                .into_iter()
                                .position(|t| t == selected_task_status)
                                .unwrap();

                            self.selected_status_task_index = self
                                .selected_status_task_index
                                .clone()
                                .with_selected(Some(index));

                            self.view_mode = ViewMode::ChangeStatusTask
                        }
                        Char('r') => {
                            let current_task = &self.projects
                                [self.selected_project_index.selected().unwrap()]
                            .tasks[self.selected_task_index.selected().unwrap()];

                            input = input.clone().with_value(current_task.title.clone());

                            self.view_mode = ViewMode::RenameTask
                        }
                        Char('n') => self.view_mode = ViewMode::AddTask,
                        Char('q') => return Ok(()),
                        Down => self.next(&items),
                        Up => self.previous(&items),
                        _ => {}
                    },
                    ViewMode::RenameTask => match key.code {
                        Enter => {
                            Task::rename(self, &mut items, input.value());
                            self.view_mode = ViewMode::ViewTasks;
                            input.reset()
                        }
                        Esc => {
                            self.view_mode = ViewMode::ViewTasks;
                            input.reset()
                        }
                        Char('q') => return Ok(()),
                        _ => {
                            input.handle_event(&Event::Key(key));
                        }
                    },
                    ViewMode::ChangeStatusTask => match key.code {
                        Enter => {
                            Task::change_status(
                                self,
                                &mut items,
                                TASK_STATUSES[self.selected_status_task_index.selected().unwrap()],
                            );
                            self.view_mode = ViewMode::ViewTasks;
                            self.selected_status_task_index =
                                ListState::default().with_selected(Some(0))
                        }
                        Down => self.next(&status_items),
                        Up => self.previous(&status_items),
                        Esc => self.view_mode = ViewMode::ViewTasks,
                        Char('q') => return Ok(()),
                        _ => {}
                    },
                    ViewMode::AddTask => match key.code {
                        Enter => {
                            Task::create(self, &mut items, input.value());
                            self.view_mode = ViewMode::ViewTasks
                        }
                        Esc => self.view_mode = ViewMode::ViewTasks,
                        _ => {
                            input.handle_event(&Event::Key(key));
                        }
                    },
                }
            }
        }
    }

    fn render(
        &mut self,
        f: &mut Frame,
        area: Rect,
        input: &Input,
        items: &Vec<ListItem>,
        status_items: &Vec<ListItem>,
    ) {
        // Create a space for header, todo list and the footer.
        let vertical = Layout::vertical([
            Constraint::Min(5),
            Constraint::Percentage(80),
            Constraint::Min(5),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        if self.view_mode == ViewMode::RenameTask || self.view_mode == ViewMode::RenameProject {
            Ui::create_input("Rename", f, area, input)
        }

        if self.view_mode == ViewMode::AddTask {
            Ui::create_input("Add new", f, area, input)
        }

        if self.view_mode == ViewMode::ChangeStatusTask {
            let area = Ui::create_rect_area(20, 5, area);

            let block = Block::bordered().title("Status");

            let list_widget = List::new(status_items.clone())
                .highlight_style(Style::default().add_modifier(Modifier::BOLD))
                .highlight_symbol("> ")
                .highlight_spacing(HighlightSpacing::Always)
                .block(block);

            f.render_widget(Clear, area);
            f.render_stateful_widget(list_widget, area, self.use_state())
        }

        let block: Block = match self.view_mode {
            ViewMode::ViewTasks | ViewMode::ChangeStatusTask => {
                let project_title = self.projects[self.selected_project_index.selected().unwrap()]
                    .title
                    .clone();

                Block::bordered().title(project_title)
            }
            _ => Block::bordered(),
        };

        // Iterate through all elements in the `items` and stylize them.
        let items = items.clone();

        // Create a List from all list items and highlight the currently selected one
        let items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always)
            .block(block);

        let footer = Paragraph::new(format!(
            "{:?} - Task: {:?}",
            self.view_mode, self.selected_task_index
        ))
        .block(Block::bordered().gray());

        if self.view_mode != ViewMode::ChangeStatusTask {
            f.render_stateful_widget(items, rest_area, self.use_state());
        } else {
            f.render_widget(items, rest_area)
        }

        f.render_widget(footer, footer_area);
        f.render_widget(Block::bordered().gray(), header_area);
    }

    fn next(&mut self, items: &Vec<ListItem>) -> () {
        let i = match self.use_state().selected() {
            Some(i) => {
                if i >= items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };

        self.use_state().select(Some(i))
    }

    fn previous(&mut self, items: &Vec<ListItem>) {
        let i = match self.use_state().selected() {
            Some(i) => {
                if i == 0 {
                    items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };

        self.use_state().select(Some(i))
    }

    fn use_state(&mut self) -> &mut ListState {
        match self.view_mode {
            ViewMode::ViewProjects => return &mut self.selected_project_index,
            ViewMode::RenameProject => return &mut self.selected_project_index,

            ViewMode::ViewTasks => return &mut self.selected_task_index,
            ViewMode::RenameTask => return &mut self.selected_task_index,
            ViewMode::ChangeStatusTask => return &mut self.selected_status_task_index,
            ViewMode::AddTask => return &mut self.selected_task_index,
        };
    }
}
