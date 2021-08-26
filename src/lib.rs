#[macro_use]
pub extern crate rocket;

pub mod cli;
pub mod web;

use crate::web::NotesResponse;

pub async fn run(args: Vec<String>, web: bool) -> Option<NotesResponse> {
    use serde_json::Value;
    use std::fs;
    use std::io::Write;

    let mut args = args.into_iter();

    let binname = args.next().unwrap();
    if !binname.ends_with("notes") {
        println!("ur a pirate huh?");
    }

    let command = args.next().unwrap_or("help".to_string());

    let notes = fs::read_to_string("notes");

    if notes.is_err() {
        let mut f = fs::File::create("notes").unwrap();
        f.write(br#"{"notes": []}"#).unwrap();
    }

    let notes = fs::read_to_string("notes").unwrap();

    let mut notes: Value = serde_json::from_str(&notes)
        .expect("Parsing error. try removing `notes` and then run `notes help`");

    if web {
        Some(match command.as_str() {
            "all" => web::all::all(notes),

            "set" => {
                let label = args.next();
                let note = args.next();

                if label.as_ref().and(note.as_ref()).is_some() {
                    let label = label.unwrap();
                    let note = note.unwrap();

                    web::set::set(&mut notes, label, note)
                } else {
                    NotesResponse::CommandError
                }
            }

            "get" => {
                let label = args.next().unwrap();

                web::get::get(notes, label)
            }

            "remove" => {
                let label = args.next().unwrap();

                web::remove::remove(&mut notes, label)
            }

            _ => NotesResponse::InvalidCommand,
        })
    } else {
        match command.as_str() {
            "help" => cli::help::help(),

            "all" => cli::all::all(notes),

            "set" => {
                let label = args.next().unwrap();
                let note = args.next().unwrap();

                cli::set::set(&mut notes, label, note)
            }

            "get" => {
                let label = args.next().unwrap();

                cli::get::get(notes, label)
            }

            "remove" => {
                let label = args.next().unwrap();

                cli::remove::remove(&mut notes, label);
            }

            "serve" => {
                cli::serve::serve().await;
            }

            _ => {
                println!("use `notes help` for info on how to use this");
            }
        };
        None
    }
}
