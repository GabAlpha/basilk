use std::{
    error::Error,
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
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Default, PartialEq)]
enum ViewMode {
    #[default]
    Project,
    Tasks,
    EditTask,
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    status: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct Project {
    title: String,
    tasks: Vec<Task>,
}

struct App {
    selected_project_index: ListState,
    selected_task_index: ListState,
    view_mode: ViewMode,
    selected_project_name: Option<String>,
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
            selected_project_name: None,
            selected_project_index: ListState::default().with_selected(Some(0)),
            selected_task_index: ListState::default().with_selected(Some(0)),
            view_mode: ViewMode::default(),
        }
    }

    fn read_json() -> Vec<Project> {
        let json = fs::read_to_string("main2.json").unwrap();
        return from_str::<Vec<Project>>(&json).unwrap();
    }

    fn load_projects(&mut self, items: &mut Vec<ListItem>, projects: &Vec<Project>) {
        items.clear();

        for project in projects.iter() {
            items.push(ListItem::from(project.title.clone()))
        }

        self.view_mode = ViewMode::Project
    }

    fn load_tasks(&mut self, items: &mut Vec<ListItem>, tasks: &Vec<Task>) {
        items.clear();

        fn get_task_status_color(status: &String) -> ratatui::prelude::Color {
            match status.as_str() {
                "Done" => return Color::Green,
                "OnGoing" => return Color::Yellow,
                "UpNext" => return Color::Blue,
                _ => return Color::Gray,
            }
        }

        for task in tasks.iter() {
            let line = Line::from(vec![
                Span::styled(
                    format!("[{}] ", task.status),
                    Style::new().fg(get_task_status_color(&task.status)),
                ),
                Span::raw(task.title.clone()),
            ]);

            items.push(ListItem::from(line))
        }

        self.view_mode = ViewMode::Tasks
    }

    fn run(&mut self, mut terminal: Terminal<impl Backend>) -> io::Result<()> {
        let mut items: Vec<ListItem> = vec![];
        let projects = App::read_json();

        self.load_projects(&mut items, &projects);

        loop {
            terminal.draw(|f| self.render(f, f.size(), &items))?;

            if let Event::Key(key) = event::read()? {
                use KeyCode::*;
                match self.view_mode {
                    ViewMode::Tasks => match key.code {
                        Esc | Left => {
                            self.selected_project_name = None;
                            self.load_projects(&mut items, &projects)
                        }
                        Enter => self.view_mode = ViewMode::EditTask,
                        _ => match key.code {
                            Char('q') => return Ok(()),
                            Down => self.next(&items),
                            Up => self.previous(&items),
                            _ => {}
                        },
                    },
                    ViewMode::EditTask => match key.code {
                        Enter => self.view_mode = ViewMode::Tasks,
                        _ => match key.code {
                            Char('q') => return Ok(()),
                            _ => {}
                        },
                    },
                    ViewMode::Project => match key.code {
                        Enter | Right => {
                            let tasks =
                                &projects[self.selected_project_index.selected().unwrap()].tasks;
                            self.selected_project_name = Some(
                                projects[self.selected_project_index.selected().unwrap()]
                                    .title
                                    .clone(),
                            );

                            self.load_tasks(&mut items, &tasks);
                            self.selected_task_index.select(Some(0))
                        }
                        _ => match key.code {
                            Char('q') => return Ok(()),
                            Down => self.next(&items),
                            Up => self.previous(&items),
                            _ => {}
                        },
                    },
                }
            }
        }
    }

    fn render(&mut self, f: &mut Frame, area: Rect, items: &Vec<ListItem>) {
        // Create a space for header, todo list and the footer.
        let vertical = Layout::vertical([
            Constraint::Min(5),
            Constraint::Percentage(80),
            Constraint::Min(5),
        ]);
        let [header_area, rest_area, footer_area] = vertical.areas(area);

        // Iterate through all elements in the `items` and stylize them.
        let items = items.clone();

        // Create a List from all list items and highlight the currently selected one
        let mut items = List::new(items)
            .highlight_style(Style::default().add_modifier(Modifier::BOLD))
            .highlight_symbol("> ")
            .highlight_spacing(HighlightSpacing::Always);

        if self.selected_project_name.is_some() {
            items = items.block(
                Block::bordered()
                    .gray()
                    .title(self.selected_project_name.clone().unwrap().bold()),
            )
        } else {
            items = items.block(Block::bordered().gray())
        }

        let footer = Paragraph::new(self.selected_task_index.selected().unwrap().to_string())
            .block(Block::bordered().gray());

        // We can now render the item list
        // (look careful we are using StatefulWidget's render.)
        // ratatui::widgets::StatefulWidget::render as stateful_render
        f.render_stateful_widget(items, rest_area, self.use_state());
        f.render_widget(footer, footer_area);
        f.render_widget(Block::bordered().gray(), header_area);
    }

    fn next(&mut self, items: &Vec<ListItem>) {
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
            ViewMode::Project => return &mut self.selected_project_index,
            ViewMode::Tasks => return &mut self.selected_task_index,
            // Show the task selected index state
            ViewMode::EditTask => return &mut self.selected_task_index,
        };
    }
}
