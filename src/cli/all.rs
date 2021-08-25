use pallete::{print_fg, printfg, println_fg, Color};
use serde_json::Value;

pub fn all(notes: Value) {
    let notes = &notes["notes"];
    let notes = notes.as_array().unwrap();

    for note in notes {
        println_fg!(Color::Cyan, "{}", note["label"].as_str().unwrap());
    }

    print!("You have ");

    if notes.len() == 0 {
        println!("no notes :'(");
    } else {
        print_fg!(Color::Green, "{}", notes.len());
        println!(" note{}", if notes.len() > 1 { "s" } else { "" });
    }
}
