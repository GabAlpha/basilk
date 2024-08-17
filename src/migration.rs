use serde_json::Value::{self};

pub static JSON_VERSIONS: [&str; 1] = ["6ad96"];

// TODO: Implement migrations
//                                 sha of 0.1.0    0.2.0    0.3.0   ...
// pub static JSON_VERSIONS: [&str; 3] = ["6ad96", "911fc", "55061"];

pub struct Migration;

impl Migration {
    pub fn get_migrations(version: &str, _original_json: Vec<Value>) -> Vec<(&str, String)> {
        // Mapper between json version and the relative migration
        let mapper: Vec<(&str, String)> = vec![
            ("6ad96", "".to_string()),
            // ("911fc", Migration::add_priority(original_json)),
        ];

        // The start index where the migration are picked
        let start_index = mapper
            .clone()
            .into_iter()
            .position(|(key, _val)| key == version);

        if start_index.is_none() {
            return vec![];
        }

        let all_migrations: Vec<(&str, String)> = mapper.into_iter().collect();

        // Slice for pick only the useful migration
        return all_migrations[(start_index.unwrap() + 1)..].to_vec();
    }

    // Migrations
    // TODO: Implement migrations

    // fn add_priority(original_json: Vec<Value>) -> String {
    //     let mut internal_json = original_json.clone();

    //     let new_json: Vec<Map<String, Value>> = internal_json
    //         .iter_mut()
    //         .map(|p| {
    //             // Get all tasks from each project and convert into "Map" type from serde
    //             // in order to do some operations with the json
    //             // Vec = Array ; Map = Object
    //             let mut tasks = serde_json::from_value::<Vec<Map<String, Value>>>(
    //                 p.get("tasks").unwrap().clone(),
    //             )
    //             .unwrap();

    //             // Add to each task a new key value (i.e. {priority: 0})
    //             tasks.iter_mut().for_each(|t| {
    //                 // Entry and or_insert methods are used for add a new key
    //                 // cf. https://docs.rs/serde_json/latest/serde_json/map/enum.Entry.html#method.or_insert
    //                 t.entry("priority").or_insert(json!(0));
    //             });

    //             // Convert "p" into the "Map" type from serde in order to do some operations with the json
    //             let mut project = serde_json::from_value::<Map<String, Value>>(p.clone()).unwrap();

    //             // Replace the "tasks" key with the new one
    //             // Insert method is used for replace a new with a new value
    //             // cf. https://docs.rs/serde_json/latest/serde_json/map/struct.Map.html#method.insert
    //             project.insert("tasks".to_string(), json!(tasks)).unwrap();

    //             return project;
    //         })
    //         .collect();

    //     return to_string(&new_json).unwrap();
    // }
}
