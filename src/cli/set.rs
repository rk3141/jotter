use pallete::{printfg, println_fg, Color};
use serde_json::{json, Value};

pub fn set(notes: &mut Value, label: String, note: String) {
    let note: Value = serde_json::from_value(json!({ "label": label,"data": note })).unwrap();

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

    let data_to_write = if std::env::var("PRETTY").is_ok() {
        serde_json::ser::to_string_pretty(&notes).unwrap()
    } else {
        serde_json::ser::to_string(&notes).unwrap()
    };

    std::fs::write("notes", data_to_write).unwrap();

    println_fg!(Color::Green, "Set label sucessfully");
}
