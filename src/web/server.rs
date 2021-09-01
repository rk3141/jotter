use std::path::{Path, PathBuf};

use rocket::fs::NamedFile;
use rocket::http::ContentType;
use serde_json::Value;

use rocket::serde::{json::Json, Deserialize, Serialize};

use crate::rocket;
use crate::web::NotesResponse;

#[get("/")]
async fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("public/index.html")).await.ok()
}

#[get("/static/<file..>")]
async fn static_file(file: PathBuf) -> Option<NamedFile> {
    let result = NamedFile::open(Path::new("public/").join(&file)).await.ok();
    if result.is_none() {
        let result = NamedFile::open(Path::new("~/.jotter/public/").join(file)).await.ok();
        result
    }
    else {
        result
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Command {
    command: Vec<String>,
}

#[post("/run", format = "json", data = "<command>")]
async fn api_run(command: Json<Command>) -> (ContentType, String) {
    let command = command.into_inner();
    let command = command.command;
    let command: Vec<&String> = command.iter().map(|v| v).collect();

    let response = crate::run(
        vec!["web-jotter".to_string()]
            .iter()
            .chain(command)
            .map(|v| v.clone())
            .collect(),
        true,
    )
    .await
    .unwrap();

    (
        ContentType::JSON,
        match response {
            NotesResponse::NoNotes => "null".to_string(),
            NotesResponse::SomeNotes(notes) => {
                let notes: Vec<Value> = notes.iter().map(|value| value["label"].clone()).collect();
                let notes = serde_json::to_value(notes).unwrap();

                notes.to_string()
            }

            NotesResponse::SetNoteSuccess => "true".to_string(),
            NotesResponse::SetNoteLabelAlreadyExist => "null".to_string(),

            NotesResponse::GetNoteNotFound => "null".to_string(),
            NotesResponse::GetNoteFound(note) => format!("{{ \"data\": {:?} }}", note),

            NotesResponse::RemoveNoteDoesntExist => "null".to_string(),
            NotesResponse::RemoveSucess => "true".to_string(),

            NotesResponse::InvalidCommand => "{ \"error\": \"unkown command\" }".to_string(),
            NotesResponse::CommandError => "{ \"syntax_error\": \"true\" }".to_string(),
        },
    )
}

pub async fn serve() {
    rocket::build()
        .mount("/", routes![index, static_file])
        .mount("/api", routes![api_run])
        .launch()
        .await
        .unwrap();
}
