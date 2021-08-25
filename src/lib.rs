#[macro_use]
pub extern crate rocket;

pub mod cli;
pub mod web;

pub async fn run(args: Vec<String>, web: bool) {
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
        println!("Web version not ready yet")
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
        }
    }
}
