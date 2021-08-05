use clap::{App, Arg};

mod model;
mod utils;

static FIRST_EMOJI: u32 = 127744;

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
                .help("Add new icon to tag")
                .takes_value(true),
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
                .help("Add new tag")
                .takes_value(true),
        )
        .get_matches();

    let tag = matches.value_of("tag").unwrap_or("");
    let add = matches.value_of("add").unwrap_or("");
    let new_tag = matches.value_of("new").unwrap_or("");
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
    if add != "" && tag != "" {
        let mut tags: Vec<model::EmojiTag>;
        let mut found_tag = false;
        let mut tag_index: Option<usize> = None;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            tags = serde_json::from_str(&tags_raw)?;

            for (index, emoji) in tags.iter().enumerate() {
                if emoji.tag == tag {
                    found_tag = true;
                    tag_index = Some(index);
                    break;
                }
            }

            if !found_tag {
                println!("Tag Not found");
                println!("Use emoji-tag -n {} to create tag", tag);
            } else {
                let utf32 = add.chars().nth(0).unwrap() as u32;

                if tags[tag_index.unwrap()].emojis.len() == 0 {
                    tags[tag_index.unwrap()].emojis += &(utf32 - FIRST_EMOJI).to_string();
                } else {
                    tags[tag_index.unwrap()].emojis = tags[tag_index.unwrap()].emojis.to_string()
                        + ","
                        + &(utf32 - FIRST_EMOJI).to_string();
                }

                let json_tag = serde_json::to_string(&tags)?;
                utils::save(&json_tag, "emoji-tag.bin");
            }
        } else {
            println!("Not found database")
        }
    } else if remove_emoji != "" && tag != "" {
        let mut tags: Vec<model::EmojiTag>;
        let mut found_tag = false;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            tags = serde_json::from_str(&tags_raw)?;

            for (index, emoji) in tags.iter().enumerate() {
                if emoji.tag == tag {
                    found_tag = true;

                    let mut all_emoji: Vec<&str> = emoji.emojis.split(",").collect();
                    let mut match_index: Option<usize> = None;

                    let mut remove_emoji32 = remove_emoji.chars().nth(0).unwrap() as u32;
                    remove_emoji32 -= FIRST_EMOJI;

                    for (emoji_index, emoji_icon) in all_emoji.iter().enumerate() {
                        if emoji_icon == &remove_emoji32.to_string() {
                            match_index = Some(emoji_index);
                        }
                    }

                    all_emoji.remove(match_index.unwrap());
                    tags[index].emojis = all_emoji.join(",");
                    break;
                }
            }

            if !found_tag {
                println!("Tag Not found");
            } else {
                let json_tag = serde_json::to_string(&tags)?;
                utils::save(&json_tag, "emoji-tag.bin");
            }
        } else {
            println!("Not found database")
        }
    } else if remove_all_emojis && tag != "" {
        let mut tags: Vec<model::EmojiTag>;
        let mut found_tag = false;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            tags = serde_json::from_str(&tags_raw)?;

            for (index, emoji) in tags.iter().enumerate() {
                if emoji.tag == tag {
                    found_tag = true;

                    tags[index].emojis = "".to_string();
                    break;
                }
            }

            if !found_tag {
                println!("Tag Not found");
            } else {
                let json_tag = serde_json::to_string(&tags)?;
                utils::save(&json_tag, "emoji-tag.bin");
            }
        } else {
            println!("Not found database")
        }
    } else if tag != "" {
        let tags: Vec<model::EmojiTag>;
        let mut found_tag = false;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            tags = serde_json::from_str(&tags_raw)?;

            for emoji in tags.iter() {
                if emoji.tag == tag {
                    found_tag = true;

                    if emoji.emojis == "" {
                        println!("Empty");
                    } else {
                        utils::print_list_emojis(&emoji.emojis);
                    }
                    break;
                }
            }

            if !found_tag {
                println!("Tag Not found");
                println!("Use emoji-tag -n {} to create tag", tag);
            }
        } else {
            println!("Not found database")
        }
    } else if new_tag != "" {
        let mut tags: Vec<model::EmojiTag>;
        let mut found_tag = false;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");

            if tags_raw != "" {
                tags = serde_json::from_str(&tags_raw)?;
                for emoji in tags.iter() {
                    if emoji.tag == new_tag {
                        found_tag = true;
                        println!("Tag is existed");
                        break;
                    }
                }
            } else {
                tags = vec![];
            }

            if !found_tag {
                let new_emoji_tag = model::EmojiTag {
                    tag: new_tag.to_string(),
                    emojis: "".to_string(),
                };

                tags.push(new_emoji_tag);

                let json_tag = serde_json::to_string(&tags)?;
                utils::save(&json_tag, "emoji-tag.bin");
            }
        } else {
            let new_emoji_tag = model::EmojiTag {
                tag: new_tag.to_string(),
                emojis: "".to_string(),
            };

            tags = vec![new_emoji_tag];

            let json_tag = serde_json::to_string(&tags)?;
            utils::save(&json_tag, "emoji-tag.bin");
        }
    } else if list {
        let tags: Vec<model::EmojiTag>;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            if tags_raw == "" {
                println!("Empty")
            } else {
                tags = serde_json::from_str(&tags_raw)?;

                for emoji in tags.iter() {
                    print!("{} ", emoji.tag);
                }
            }
        } else {
            println!("Empty");
        }
    } else if remove_tag != "" {
        let mut tags: Vec<model::EmojiTag>;
        let mut found_tag = false;

        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            let mut tag_index: Option<usize> = None;
            tags = serde_json::from_str(&tags_raw)?;

            for (index, emoji) in tags.iter().enumerate() {
                if emoji.tag == remove_tag {
                    found_tag = true;
                    tag_index = Some(index);
                    break;
                }
            }

            if !found_tag {
                println!("Tag not found")
            } else {
                tags.remove(tag_index.unwrap());

                let json_tag = serde_json::to_string(&tags)?;
                utils::save(&json_tag, "emoji-tag.bin");
            }
        }
    } else if remove_all_tags {
        utils::save(&"".to_string(), "emoji-tag.bin");
    } else {
        println!("Wrong arguments")
    }

    Ok(())
}
