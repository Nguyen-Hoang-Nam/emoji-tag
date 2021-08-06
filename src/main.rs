use clap::{App, Arg};

mod commands;
mod model;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = App::new("emoji-tag")
        .version("1.1.0")
        .author("N.H Nam <nguyenhoangnam.dev@gmail.com>")
        .about("Get emoji by tag")
        .arg(
            Arg::with_name("tag")
                .short("t")
                .long("tag")
                .help("Choose tag")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("list")
                .short("l")
                .long("list")
                .help("List all tags"),
        )
        .arg(
            Arg::with_name("color")
                .short("c")
                .long("color")
                .help("Choose color for emoji")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .multiple(true)
                .help("Add new icon to tag")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("all")
                .short("A")
                .long("all")
                .help("Show all emojis, and tags"),
        )
        .arg(
            Arg::with_name("remove-emoji")
                .short("r")
                .long("remove-emoji")
                .help("Remove emoji")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("remove-tag")
                .short("R")
                .long("remove-tag")
                .help("Remove tag")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("remove-all-emojis")
                .long("remove-all-emojis")
                .help("Remove all emojis"),
        )
        .arg(
            Arg::with_name("remove-all-tags")
                .long("remove-all-tags")
                .help("Remove all tags"),
        )
        .arg(
            Arg::with_name("new")
                .short("n")
                .long("new")
                .multiple(true)
                .help("Add new tag")
                .takes_value(true),
        )
        .get_matches();

    let tag = matches.value_of("tag").unwrap_or("");
    let add: Vec<_> = match matches.values_of("add") {
        Some(values) => values.collect(),
        None => vec![],
    };

    let all = match matches.occurrences_of("all") {
        0 => false,
        _ => true,
    };

    let new_tag: Vec<_> = match matches.values_of("new") {
        Some(values) => values.collect(),
        None => vec![],
    };
    let list = match matches.occurrences_of("list") {
        0 => false,
        _ => true,
    };

    let remove_emoji = matches.value_of("remove-emoji").unwrap_or("");
    let remove_all_emojis = match matches.occurrences_of("remove-all-emojis") {
        0 => false,
        _ => true,
    };

    let remove_tag = matches.value_of("remove-tag").unwrap_or("");
    let remove_all_tags = match matches.occurrences_of("remove-all-tags") {
        0 => false,
        _ => true,
    };

    // Add emoji to a specific tag
    if add.len() > 0 && tag != "" {
        commands::add_emoji(&add, tag).unwrap();

    // Remove a specific emoji in a specific tag
    } else if remove_emoji != "" && tag != "" {
        commands::remove_emoji_by_tag(remove_emoji, tag).unwrap();

    // Remove all emojis in a specific tag
    } else if remove_all_emojis && tag != "" {
        commands::remove_all_emoji_by_tag(tag).unwrap();

    // Get all emojis of a specific tag
    } else if tag != "" {
        commands::get_tag(tag).unwrap();

    // Create a new tag
    } else if new_tag.len() > 0 {
        commands::create_tag(&new_tag).unwrap();

    // Show all tag's name
    } else if list {
        commands::list_tab().unwrap();

    // Remove a specific tag
    } else if remove_tag != "" {
        commands::remove_tag_by_tag(remove_tag).unwrap();

    // Remove all tags
    } else if remove_all_tags {
        commands::remove_all_tag();

    // Show all emojis, and tags
    } else if all {
        commands::get_all().unwrap();
    } else {
        println!("Wrong arguments")
    }

    Ok(())
}
