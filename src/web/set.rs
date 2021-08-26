use serde_json::{json, Value};

use crate::web::NotesResponse;

pub fn set(notes: &mut Value, label: String, note: String) -> NotesResponse {
    let note: Value = serde_json::from_value(json!({ "label": label,"data": note })).unwrap();

    let mut notes_arr = notes["notes"].clone().as_array().unwrap().clone();

    if notes_arr
        .iter()
        .fold(false, |acc, v| acc || v["label"].as_str().unwrap() == label)
    {
        NotesResponse::SetNoteLabelAlreadyExist
    } else {
        notes_arr.push(note);

        notes["notes"] = serde_json::value::to_value(notes_arr).unwrap();

        let data_to_write = if std::env::var("PRETTY").is_ok() {
            serde_json::ser::to_string_pretty(&notes).unwrap()
        } else {
            serde_json::ser::to_string(&notes).unwrap()
        };

        std::fs::write("notes", data_to_write).unwrap();

        NotesResponse::SetNoteSuccess
    }
}
