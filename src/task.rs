use serde::{ Serialize, Deserialize };
use std::fs::{ OpenOptions };
use std::io::{ Read, Write };

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub(crate) title: String,
    pub(crate) done: bool,
}

pub fn load_tasks() -> Vec<Task> {
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open("tasks.json")
        .expect("Failed to open file");

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();
    if content.is_empty() {
        return vec![];
    }

    serde_json::from_str(&content).unwrap()
}

pub fn save_tasks(tasks: &[Task]) {
    let json = serde_json::to_string_pretty(tasks).unwrap();
    let mut file = OpenOptions::new().write(true).truncate(true).open("tasks.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
}
