use pallete::{print_fg, printfg, println_fg, Color};

pub fn help() {
    print_fg!(Color::Cyan, "Jotter");
    print!(" - ");
    print_fg!(Color::Yellow, "A very ");

    print_fg!(Color::Red, "(un)secure");

    println_fg!(Color::Yellow, " tool for writing notes\n");

    println_fg!(Color::Green, "Commands:");
    let commands = vec!["help", "all", "set", "remove", "get"];

    let command_help_messages = vec![
        "help for using this tool",
        "list all your notes",
        "set a label with content (notes set label 'content')",
        "remove label",
        "get a label",
    ];

    let max_command_len = commands
        .iter()
        .fold(0, |acc, v| if v.len() > acc { v.len() } else { acc });

    commands
        .iter()
        .zip(command_help_messages.iter())
        .for_each(|(command, help)| {
            print_fg!(Color::Yellow, "  `{}`", command);

            print!(
                "{}: ",
                std::iter::repeat(" ")
                    .take(max_command_len - command.len())
                    .collect::<String>()
            );

            println_fg!(Color::Cyan, "{}", help);
        })
    //     println!(
    //         "Notes - (A very (un)secure tool for writing notes on the cli)
    // "
    //     )
}
