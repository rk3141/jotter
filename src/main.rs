use std::{
    env::{self, args},
    fs,
    io::Write,
};

use pallete::*;
use serde_json::{json, Value};

fn main() {
    let args = args();
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

    match command.as_str() {
        "help" => println!(
            "Notes - (A very (un)secure tool for writing notes on the cli)
  Commands:
    `help`: help for using this tool
    `all`: List all your notes
"
        ),

        "all" => {
            let notes = &notes["notes"];
            let notes = notes.as_array().unwrap();

            for note in notes {
                pallete::println_fg!(Color::Cyan, "{}", note["label"].as_str().unwrap());
            }

            print!("You have ");
            print_fg!(Color::Green, "{}", notes.len());

            println!(" note{}", if notes.len() > 1 { "s" } else { "" });
        }

        "set" => {
            let label = args.next().unwrap();
            let note = args.next().unwrap();

            let note: Value =
                serde_json::from_value(json!({ "label": label,"data": note })).unwrap();

            let mut notes_arr = notes["notes"].clone().as_array().unwrap().clone();

            if notes_arr
                .iter()
                .fold(false, |acc, v| acc || v["label"].as_str().unwrap() == label)
            {
                println_fg!(Color::Red, "Label already exists");

                return;
            }

            notes_arr.push(note);

            notes["notes"] = serde_json::value::to_value(notes_arr).unwrap();

            let data_to_write = {
                if env::var("PRETTY").is_ok() {
                    serde_json::ser::to_string_pretty(&notes).unwrap()
                } else {
                    serde_json::ser::to_string(&notes).unwrap()
                }
            };

            fs::write("notes", data_to_write).unwrap();

            println_fg!(Color::Green, "Set label sucessfully");
        }

        "get" => {
            let label = args.next().unwrap();

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
                println_fg!(Color::Red, "Label doesnt exist. maybe a typo");
            } else {
                let note = note["data"].as_str().unwrap();

                print_fg!(Color::Green, "{}", label);
                println!(":");
                println!("{}", note);
            }
        }

        "remove" => {
            let label = args.next().unwrap();

            let mut notes_arr = notes["notes"].clone().as_array().unwrap().clone();

            let plen = notes_arr.len();

            notes_arr = notes_arr
                .iter()
                .filter(|&note| note["label"] != label)
                .map(|v| v.clone())
                .collect();

            let nlen = notes_arr.len();

            if plen == nlen {
                println_fg!(Color::Red, "Label doesnt exist. maybe a typo");
                return;
            }

            notes["notes"] = serde_json::value::to_value(notes_arr).unwrap();

            let data_to_write = {
                if env::var("PRETTY").is_ok() {
                    serde_json::ser::to_string_pretty(&notes).unwrap()
                } else {
                    serde_json::ser::to_string(&notes).unwrap()
                }
            };

            fs::write("notes", data_to_write).unwrap();
            println_fg!(Color::Red, "Removed label {:?}", label);
        }

        _ => {
            println!("use `notes help` for info on how to use this");
        }
    }
}
