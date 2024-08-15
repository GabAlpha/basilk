use std::fs;

use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{config::PATH_JSON, App, ViewMode};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Task {
    pub title: String,
    pub status: String,
}

impl Task {
    fn get_status_color(status: &String) -> ratatui::prelude::Color {
        match status.as_str() {
            "Done" => return Color::Green,
            "OnGoing" => return Color::Yellow,
            "UpNext" => return Color::Blue,
            _ => return Color::Gray,
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

        app.view_mode = ViewMode::Tasks
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
}
