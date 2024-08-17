use std::{
    collections::HashMap,
    error::Error,
    fs::{self, File},
    io::Write,
    path::Path,
    sync::Mutex,
};

use serde_json::{from_str, to_string};

use crate::{config::JSON_VERSIONS, project::Project};

pub struct Json;

static VERSION: Mutex<String> = Mutex::new(String::new());

impl Json {
    fn get_json_path(version: String) -> String {
        format!("{}.json", version)
    }

    pub fn check() -> Result<String, Box<dyn Error>> {
        let mut json_version: Vec<&str> = JSON_VERSIONS
            .into_iter()
            .filter(|version| Path::new(&Json::get_json_path(version.to_string())).is_file())
            .collect();

        if json_version.is_empty() {
            let last_json_version = JSON_VERSIONS.last().unwrap();
            let path = Json::get_json_path(last_json_version.to_string());

            let mut file = File::create(path).unwrap();
            let _ = file.write_all(b"[]");

            json_version = vec![last_json_version]
        }

        let mut version = VERSION.lock().unwrap();
        version.push_str(json_version[0]);

        Ok(String::from(json_version[0]))
    }

    pub fn read() -> Vec<Project> {
        let version = VERSION.lock().unwrap().to_string();
        let path = Json::get_json_path(version);

        let json = fs::read_to_string(path).unwrap();
        return from_str::<Vec<Project>>(&json).unwrap();
    }

    pub fn write(projects: Vec<Project>) {
        let version = VERSION.lock().unwrap().to_string();
        let path = Json::get_json_path(version);

        fs::write(Json::get_json_path(path), to_string(&projects).unwrap()).unwrap();
    }
}
