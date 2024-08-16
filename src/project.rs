use std::fs;

use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{
    config::PATH_JSON,
    task::{Task, TASK_STATUS_DONE},
    App,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Project {
    fn get_indicator_done_tasks_color(percentage: usize) -> ratatui::prelude::Color {
        match percentage {
            p if p == 0 => return Color::DarkGray,
            p if p >= 25 && p <= 50 => return Color::LightMagenta,
            p if p >= 50 && p < 100 => return Color::LightYellow,
            p if p == 100 => return Color::LightGreen,
            _ => return Color::White,
        }
    }
    pub fn load(app: &mut App, items: &mut Vec<ListItem>) {
        items.clear();

        for project in app.projects.iter() {
            let tasks = &project.tasks;

            let done_tasks: Vec<Task> = tasks
                .clone()
                .into_iter()
                .filter(|t| t.status == TASK_STATUS_DONE)
                .collect();

            let percentage = if tasks.len() == 0 {
                0
            } else {
                (done_tasks.len() * 100) / tasks.len()
            };

            let lines = vec![Line::from(vec![
                Span::raw(format!("[{}/{}] ", done_tasks.len(), tasks.len(),)).style(
                    Style::default()
                        .add_modifier(Modifier::ITALIC)
                        .fg(Project::get_indicator_done_tasks_color(percentage)),
                ),
                Span::raw(project.title.clone()),
            ])];

            items.push(ListItem::from(lines))
        }
    }

    pub fn get_current_project(app: &mut App) -> &Project {
        return &app.projects[app.selected_project_index.selected().unwrap()];
    }

    pub fn create(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        if value.is_empty() {
            return;
        }

        let new_project = Project {
            title: value.to_string(),
            tasks: vec![],
        };

        let mut internal_projects = app.projects.clone();
        internal_projects.push(new_project);

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Project::load(app, items)
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].title = value.to_string();

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Project::load(app, items)
    }

    pub fn delete(app: &mut App, items: &mut Vec<ListItem>) {
        let mut internal_projects = app.projects.clone();

        internal_projects.remove(app.selected_project_index.selected().unwrap());

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Project::load(app, items)
    }
}
