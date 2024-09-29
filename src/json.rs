use std::{
    error::Error,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    sync::Mutex,
};

use serde_json::{from_str, to_string, Value};

use crate::{
    migration::{Migration, JSON_VERSIONS},
    project::Project,
};

pub struct Json;

static DIR_CONFIG_NAME: &str = env!("CARGO_PKG_NAME");
static VERSION: Mutex<String> = Mutex::new(String::new());

impl Json {
    pub fn get_dir_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap();
        path.push(DIR_CONFIG_NAME);

        return path;
    }

    fn get_json_path(version: String) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(Json::get_dir_path().as_path());
        path.push(format!("{version}.json"));

        return path;
    }

    pub fn check() -> Result<bool, Box<dyn Error>> {
        fs::create_dir_all(Json::get_dir_path())?;

        // Create the state to save the json version
        let mut version_state = VERSION.lock().unwrap();

        // Pick the version from the internal file
        let mut json_version_from_file: Vec<&str> = JSON_VERSIONS
            .into_iter()
            .filter(|version| Path::new(&Json::get_json_path(version.to_string())).is_file())
            .collect();

        // If the file doesn't exist create a new one with the last version
        if json_version_from_file.is_empty() {
            let last_json_version = JSON_VERSIONS.last().unwrap();
            let path = Json::get_json_path(last_json_version.to_string());

            let mut file = File::create(path).unwrap();
            let _ = file.write_all(b"[]");

            json_version_from_file = vec![last_json_version];
            version_state.push_str(json_version_from_file[0]);

            return Ok(false);
        }

        // Save into the internal state the last json version
        version_state.push_str(json_version_from_file[0]);

        // Read the internal file
        let path = Json::get_json_path(json_version_from_file[0].to_string());
        let json_raw = fs::read_to_string(&path).unwrap();
        let json = from_str::<Vec<Value>>(&json_raw).unwrap();

        if json.is_empty() {
            return Ok(false);
        }

        // Load all migrations
        let migrations = Migration::get_migrations(json_version_from_file[0], json);

        if migrations.is_empty() {
            return Ok(false);
        }

        // Loop thru all migrations and apply them!
        for (version, migration) in migrations.iter() {
            let path = Json::get_json_path(version_state.to_string());
            let new_path = Json::get_json_path(version.to_string());

            let new_json = migration;

            fs::write(&path, new_json).unwrap();
            fs::rename(&path, new_path)?;

            // Save into the internal state the json version of the last migration applied
            version_state.clear();
            version_state.push_str(&version)
        }

        Ok(true)
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

        fs::write(path, to_string(&projects).unwrap()).unwrap();
    }
}
