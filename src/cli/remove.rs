use pallete::{printfg, println_fg, Color};
use serde_json::Value;
use std::{env, fs, path::PathBuf};

pub fn remove(notes: &mut Value, label: String, file_path: &PathBuf) {
    let notes_arr = notes["notes"].clone().as_array().unwrap().clone();

    let note = notes_arr
        .iter()
        .find(|&note| note["label"] == label)
        .map(|v| v.clone());

    if note.is_none() {
        println_fg!(Color::Red, "Label doesnt exist. maybe a typo");
        return;
    }

    let notes_arr: Vec<Value> = notes_arr
        .iter()
        .filter(|&note| note["label"] != label)
        .map(|val| val.clone())
        .collect();

    notes["notes"] = serde_json::value::to_value(notes_arr).unwrap();

    let data_to_write = {
        if env::var("PRETTY").is_ok() {
            serde_json::ser::to_string_pretty(&notes).unwrap()
        } else {
            serde_json::ser::to_string(&notes).unwrap()
        }
    };

    fs::write(file_path, data_to_write).unwrap();
    println_fg!(Color::Red, "Removed label {:?}", label);
}
