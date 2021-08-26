use serde_json::Value;

pub mod all;
pub mod get;
pub mod help;
pub mod remove;
pub mod serve;
pub mod set;

pub mod server;

#[derive(Debug)]
pub enum NotesResponse {
    NoNotes,
    SomeNotes(Vec<Value>),

    SetNoteSuccess,
    SetNoteLabelAlreadyExist,

    GetNoteNotFound,
    GetNoteFound(String),

    RemoveSucess,
    RemoveNoteDoesntExist,

    InvalidCommand,
    CommandError,
}
