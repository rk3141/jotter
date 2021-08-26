use serde_json::Value;

use super::NotesResponse;

pub fn get(notes: Value, label: String) -> NotesResponse {
    let notes_arr = notes["notes"].clone();
    let notes_arr = notes_arr.as_array().unwrap().clone();

    let note = notes_arr.iter().fold(Value::Null, |acc, v| {
        if !v.is_null() && v["label"].as_str().unwrap() == label {
            v.clone()
        } else {
            acc
        }
    });

    if note.is_null() {
        NotesResponse::GetNoteNotFound
    } else {
        let note = note["data"].as_str().unwrap();

        NotesResponse::GetNoteFound(note.to_string())
    }
}
