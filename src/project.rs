use ratatui::{
    style::{Color, Style},
    text::{Line, Span},
    widgets::ListItem,
};
use serde::{Deserialize, Serialize};

use crate::{
    json::Json,
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

    pub fn load_items(app: &mut App, items: &mut Vec<ListItem>) {
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
                    Style::default().fg(Project::get_indicator_done_tasks_color(percentage)),
                ),
                Span::raw(project.title.clone()),
            ])];

            items.push(ListItem::from(lines))
        }
    }

    pub fn reload(app: &mut App, items: &mut Vec<ListItem>) {
        app.projects = Json::read();
        Project::load_items(app, items)
    }

    pub fn get_current(app: &mut App) -> &Project {
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

        Json::write(internal_projects);
        Project::reload(app, items)
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].title = value.to_string();

        Json::write(internal_projects);
        Project::reload(app, items)
    }

    pub fn delete(app: &mut App, items: &mut Vec<ListItem>) {
        let mut internal_projects = app.projects.clone();

        internal_projects.remove(app.selected_project_index.selected().unwrap());

        Json::write(internal_projects);
        Project::reload(app, items)
    }
}
