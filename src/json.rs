use std::fs;

use serde_json::{from_str, to_string};

use crate::{config::PATH_JSON, project::Project};

pub struct Json;

impl Json {
    pub fn read() -> Vec<Project> {
        let json = fs::read_to_string(PATH_JSON).unwrap();
        return from_str::<Vec<Project>>(&json).unwrap();
    }

    pub fn write(projects: Vec<Project>) {
        fs::write(PATH_JSON, to_string(&projects).unwrap()).unwrap();
    }
}
