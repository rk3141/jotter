use super::NotesResponse;
use serde_json::Value;

pub fn all(notes: Value) -> NotesResponse {
    let notes = &notes["notes"];
    let notes = notes.as_array().unwrap();

    if notes.len() == 0 {
        NotesResponse::NoNotes
    } else {
        NotesResponse::SomeNotes(notes.to_vec())
    }
}
