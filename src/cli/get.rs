use pallete::{print_fg, printfg, println_fg, Color};
use serde_json::Value;

pub fn get(notes: Value, label: String) {
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
        panic!("label not found");
    } else {
        let note = note["data"].as_str().unwrap();

        print_fg!(Color::Green, "{}", label);
        println!(":");
        println!("{}", note);
    }
}
