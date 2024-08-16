use std::fs;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{config::PATH_JSON, App};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub status: String,
}

pub const TASK_STATUS_DONE: &str = "Done";
pub const TASK_STATUS_ON_GOING: &str = "OnGoing";
pub const TASK_STATUS_UP_NEXT: &str = "UpNext";

pub const TASK_STATUSES: [&'static str; 3] =
    [TASK_STATUS_UP_NEXT, TASK_STATUS_ON_GOING, TASK_STATUS_DONE];

impl Task {
    fn get_status_color(status: &String) -> ratatui::prelude::Color {
        match status.as_str() {
            TASK_STATUS_DONE => return Color::Green,
            TASK_STATUS_ON_GOING => return Color::Yellow,
            TASK_STATUS_UP_NEXT => return Color::Blue,
            _ => return Color::Gray,
        }
    }

    pub fn load_statues(items: &mut Vec<ListItem>) {
        items.clear();

        for status in TASK_STATUSES {
            let span = Span::styled(
                status,
                Style::new().fg(Task::get_status_color(&status.to_string())),
            );

            items.push(ListItem::from(span))
        }
    }

    pub fn load(app: &mut App, items: &mut Vec<ListItem>) {
        let tasks = &app.projects[app.selected_project_index.selected().unwrap()].tasks;

        items.clear();

        for task in tasks.iter() {
            let line = Line::from(vec![
                Span::styled(
                    format!("[{}] ", task.status),
                    Style::new().fg(Task::get_status_color(&task.status)),
                ),
                Span::raw(task.title.clone()),
            ]);

            items.push(ListItem::from(line))
        }
    }

    pub fn create(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        if value.is_empty() {
            return;
        }

        let new_task = Task {
            title: value.to_string(),
            status: TASK_STATUS_UP_NEXT.to_string(),
        };

        let mut internal_projects = app.projects.clone();
        internal_projects[app.selected_project_index.selected().unwrap()]
            .tasks
            .push(new_task);

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Task::load(app, items)
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
        .title = value.to_string();

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Task::load(app, items)
    }

    pub fn change_status(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].tasks
            [app.selected_task_index.selected().unwrap()]
        .status = value.to_string();

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Task::load(app, items)
    }
}
