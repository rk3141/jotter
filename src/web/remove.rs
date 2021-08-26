use serde_json::Value;
use std::{env, fs};

use super::NotesResponse;

pub fn remove(notes: &mut Value, label: String) -> NotesResponse {
    let notes_arr = notes["notes"].clone().as_array().unwrap().clone();

    let note = notes_arr
        .iter()
        .find(|&note| note["label"] == label)
        .map(|v| v.clone());

    if note.is_none() {
        NotesResponse::RemoveNoteDoesntExist
    } else {
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

        fs::write("notes", data_to_write).unwrap();
        NotesResponse::RemoveSucess
    }
}
