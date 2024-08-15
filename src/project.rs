use std::fs;

use ratatui::widgets::ListItem;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

use crate::{config::PATH_JSON, task::Task, App};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub title: String,
    pub tasks: Vec<Task>,
}

impl Project {
    pub fn load(app: &mut App, items: &mut Vec<ListItem>) {
        items.clear();

        for project in app.projects.iter() {
            items.push(ListItem::from(project.title.clone()))
        }
    }

    pub fn rename(app: &mut App, items: &mut Vec<ListItem>, value: &str) {
        let mut internal_projects = app.projects.clone();

        internal_projects[app.selected_project_index.selected().unwrap()].title = value.to_string();

        fs::write(PATH_JSON, to_string(&internal_projects).unwrap()).unwrap();

        app.projects = App::read_json();
        Project::load(app, items)
    }
}
