#[macro_use]
pub extern crate rocket;

pub mod cli;
pub mod web;

use std::path::Path;

use crate::web::NotesResponse;

pub async fn run(args: Vec<String>, web: bool) -> Option<NotesResponse> {
    use serde_json::Value;
    use std::fs;
    use std::io::Write;

    let mut args = args.into_iter();

    let binname = args.next().unwrap();
    if !binname.ends_with("jotter") {
        println!("ur a pirate huh?");
    }

    let command = args.next().unwrap_or("help".to_string());

    let home_dir = std::env::var("HOME")
        .or(std::env::var("USERPROFILE"))
        .expect("Expected a home directory in $HOME or %USERPROFILE%");

    let file_path = Path::new(&home_dir).join(".notes");

    let notes = fs::read_to_string(&file_path);

    let notes = if notes.is_err() {
        let mut f = fs::File::create(&file_path).unwrap();
        f.write(br#"{"notes": []}"#).unwrap();

        r#"{"notes": []}"#.to_string()
    } else {
        notes.unwrap()
    };

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

                    web::set::set(&mut notes, label, note, &file_path)
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

                web::remove::remove(&mut notes, label, &file_path)
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

                cli::set::set(&mut notes, label, note, &file_path)
            }

            "get" => {
                let label = args.next().unwrap();

                cli::get::get(notes, label)
            }

            "remove" => {
                let label = args.next().unwrap();

                cli::remove::remove(&mut notes, label, &file_path);
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
