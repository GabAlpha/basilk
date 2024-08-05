//! # [Ratatui] Scrollbar example
//!
//! The latest version of this example is available in the [examples] folder in the repository.
//!
//! Please note that the examples are designed to be run against the `main` branch of the Github
//! repository. This means that you may not be able to compile with the latest release version on
//! crates.io, or the one that you have installed locally.
//!
//! See the [examples readme] for more information on finding examples that match the version of the
//! library you are using.
//!
//! [Ratatui]: https://github.com/ratatui-org/ratatui
//! [examples]: https://github.com/ratatui-org/ratatui/blob/main/examples
//! [examples readme]: https://github.com/ratatui-org/ratatui/blob/main/examples/README.md

#![warn(clippy::pedantic)]
#![allow(clippy::wildcard_imports)]

use std::{
    error::Error, io, process::exit, time::{Duration, Instant}, fs, str
};

use ratatui::{
    crossterm::{
        event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    prelude::*,
    widgets::*,
};
use serde::{Deserialize, Serialize};
use serde_json::from_str;

#[derive(Default, PartialEq)]
enum Mode {
    #[default]
    Project,
    Tasks
}

#[derive(Deserialize, Serialize, Debug)]
struct Task {
    title: String,
    status: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Project {
    title: String,
    tasks: Vec<Task>
}

#[derive(Default)]
struct App {
    pub vertical_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub index_selected: usize,
    pub mode: Mode
}

fn main() -> Result<(), Box<dyn Error>> {
    // setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend: CrosstermBackend<io::Stdout> = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let json = fs::read_to_string("main.json").unwrap();
    let projects = from_str::<Vec<Project>>(&json).unwrap();

    // create app and run it
    let tick_rate = Duration::from_millis(250);
    let app = App::default();
    let res = run_app(&mut terminal, app, projects, tick_rate);

    // restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
        exit(1)
    }

    Ok(())
}

fn read_projects(text: &mut Vec<Line>, projects: &Vec<Project>, app: &mut App){
    for project in projects.iter() {
        text.push(Line::from(project.title.clone()))
    }
    app.mode = Mode::Project
}

fn get_task_status_color (status: &String) -> ratatui::prelude::Color{
    match status.as_str() {
        "Done" => return Color::Green,
        "OnGoing" => return Color::Yellow,
        "UpNext" => return Color::Blue,
        _ => return Color::Gray
    }
}

fn read_tasks(text: &mut Vec<Line>, tasks: &Vec<Task>, app: &mut App){
    for task in tasks.iter() {
        let color = get_task_status_color(&task.status);
        text.push(Line::from(vec![Span::styled(format!("[{}] ", task.status), Style::new().fg(color)), Span::raw(task.title.clone())]))
    }
    app.mode = Mode::Tasks
}


fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    projects: Vec<Project>,
    tick_rate: Duration,
) -> io::Result<()> {
    let mut last_tick = Instant::now();

    let mut text: Vec<Line> = vec![];

    read_projects(&mut text, &projects, &mut app);

    loop {
        terminal.draw(|f| ui(f, &mut app, &mut text))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Enter => {
                        if Mode::Project == app.mode {
                            let tasks = &projects[app.index_selected].tasks;
                            
                            app.index_selected = 0;
                            app.vertical_scroll = 0;
    
                            text.clear();
                            read_tasks(&mut text, &tasks, &mut app)
                        }
                    },
                    KeyCode::Esc => {
                        app.index_selected = 0;
                        app.vertical_scroll = 0;

                        text.clear(); 
                        read_projects(&mut text, &projects, &mut app);
                    },
                    KeyCode::Down => {
                        if app.index_selected + 1 < text.len() {
                            app.index_selected = app.index_selected + 1;

                            // app.vertical_scroll = app.vertical_scroll.saturating_add(1);
                            // app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
                                
                        }
                        
                    }
                     KeyCode::Up => {
                        if app.index_selected != 0 {
                            app.index_selected = app.index_selected - 1;

                            app.vertical_scroll = app.vertical_scroll.saturating_sub(1);
                            app.vertical_scroll_state = app.vertical_scroll_state.position(app.vertical_scroll);
                        }
                    }
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
        }
    }
}

#[allow(clippy::too_many_lines, clippy::cast_possible_truncation)]
fn ui(f: &mut Frame, app: &mut App, text: &mut Vec<Line>) {

    let size = f.size();

    let mut internal_text = text.clone();

    let chunks = Layout::vertical([
        Constraint::Min(1),
        Constraint::Percentage(80), 
        Constraint::Min(1),
    ])
    .split(size);


    app.vertical_scroll_state = app.vertical_scroll_state.content_length(internal_text.len());

    let create_block = |title: &'static str| Block::bordered().gray().title(title.bold());

    let title = Block::new()
        .title_alignment(Alignment::Center)
        .title("title".bold());
    f.render_widget(title, chunks[0]);

    (internal_text)[app.index_selected] = (internal_text)[app.index_selected].clone().on_dark_gray();

    let paragraph = Paragraph::new(internal_text.clone())
        .gray()
        .block(create_block("Vertical scrollbar with arrows"))
        .scroll((app.vertical_scroll as u16, 0));

    f.render_widget(paragraph, chunks[1]);
    f.render_widget(Paragraph::new(format!("{} - {}", app.vertical_scroll.to_string(), chunks[2].rows().current_row)), chunks[2]);

    f.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓")),
        chunks[1],
        &mut app.vertical_scroll_state,
    );
}