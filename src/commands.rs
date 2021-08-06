use crate::model;
use crate::utils;

static FIRST_EMOJI: u32 = 127744;

pub fn add_emoji(add: &Vec<&str>, tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_emojis(add) {
        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            if tags_raw == "" {
                println!("Empty")
            } else {
                let mut tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

                match utils::find_tag(&tags, tag) {
                    Some(index) => {
                        for emoji in add.iter() {
                            let utf32 = emoji.chars().nth(0).unwrap() as u32;

                            if tags[index].emojis.len() == 0 {
                                tags[index].emojis += &(utf32 - FIRST_EMOJI).to_string();
                            } else {
                                tags[index].emojis =
                                    utils::add_emoji_sort(&tags[index].emojis, utf32);
                            }
                        }

                        let json_tag = serde_json::to_string(&tags)?;
                        utils::save(&json_tag, "emoji-tag.bin");
                    }
                    None => {
                        println!("Tag Not found");
                        println!("Use emoji-tag -n {} to create tag", tag);
                    }
                }
            }
        } else {
            println!("Not found database")
        }
    } else {
        println!("Emoji not found")
    }

    Ok(())
}

pub fn remove_emoji_by_tag(
    remove_emoji: &str,
    tag: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_emoji(remove_emoji) {
        if utils::check_file_exist("emoji-tag.bin") {
            let tags_raw = utils::load("emoji-tag.bin");
            if tags_raw == "" {
                println!("Empty")
            } else {
                let mut tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

                match utils::find_tag(&tags, tag) {
                    Some(index) => {
                        let mut all_emoji: Vec<&str> = tags[index].emojis.split(",").collect();
                        let mut match_index: Option<usize> = None;

                        let mut remove_emoji32 = remove_emoji.chars().nth(0).unwrap() as u32;
                        remove_emoji32 -= FIRST_EMOJI;

                        for (emoji_index, emoji_icon) in all_emoji.iter().enumerate() {
                            if emoji_icon == &remove_emoji32.to_string() {
                                match_index = Some(emoji_index);
                                break;
                            }
                        }

                        match match_index {
                            Some(emoji_index) => {
                                all_emoji.remove(emoji_index);
                                tags[index].emojis = all_emoji.join(",");

                                let json_tag = serde_json::to_string(&tags)?;
                                utils::save(&json_tag, "emoji-tag.bin");
                            }
                            None => {
                                println!("Emoji not found")
                            }
                        }
                    }
                    None => {
                        println!("Tag Not found");
                    }
                }
            }
        } else {
            println!("Not found database")
        }
    } else {
        println!("Emoji not found")
    }

    Ok(())
}

pub fn remove_all_emoji_by_tag(tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        if tags_raw == "" {
            println!("Empty")
        } else {
            let mut tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

            match utils::find_tag(&tags, tag) {
                Some(index) => {
                    tags[index].emojis = "".to_string();
                    let json_tag = serde_json::to_string(&tags)?;
                    utils::save(&json_tag, "emoji-tag.bin");
                }
                None => {
                    println!("Tag Not found")
                }
            }
        }
    } else {
        println!("Not found database")
    }

    Ok(())
}

pub fn get_tag(tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        if tags_raw == "" {
            println!("empty")
        } else {
            let tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

            match utils::find_tag(&tags, tag) {
                Some(index) => {
                    if tags[index].emojis == "" {
                        println!("Empty");
                    } else {
                        utils::print_list_emojis(&tags[index].emojis);
                    }
                }
                None => {
                    println!("Tag Not found");
                    println!("Use emoji-tag -n {} to create tag", tag);
                }
            }
        }
    } else {
        println!("Not found database")
    }

    Ok(())
}

pub fn create_tag(new_tag: &Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let mut tags: Vec<model::EmojiTag>;

    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        let mut non_exist_tag: Vec<&str> = vec![];

        if tags_raw != "" {
            tags = serde_json::from_str(&tags_raw)?;
            for tag in new_tag.iter() {
                let mut found_tag = false;
                for emoji in tags.iter() {
                    if &emoji.tag == tag {
                        found_tag = true;
                        break;
                    }
                }

                if !found_tag {
                    non_exist_tag.push(tag);
                }
            }
        } else {
            tags = vec![];
            for tag in new_tag.iter() {
                non_exist_tag.push(tag);
            }
        }

        if non_exist_tag.len() > 0 {
            for tag in non_exist_tag.iter() {
                let new_emoji_tag = model::EmojiTag {
                    tag: tag.to_string(),
                    emojis: "".to_string(),
                };

                tags.push(new_emoji_tag);
            }

            let json_tag = serde_json::to_string(&tags)?;
            utils::save(&json_tag, "emoji-tag.bin");
        }
    } else {
        tags = vec![];

        for tag in new_tag.iter() {
            let new_emoji_tag = model::EmojiTag {
                tag: tag.to_string(),
                emojis: "".to_string(),
            };

            tags.push(new_emoji_tag);
        }
        // let new_emoji_tag = model::EmojiTag {
        //     tag: new_tag.to_string(),
        //     emojis: "".to_string(),
        // };

        // tags = vec![new_emoji_tag];

        let json_tag = serde_json::to_string(&tags)?;
        utils::save(&json_tag, "emoji-tag.bin");
    }

    Ok(())
}

pub fn list_tab() -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        if tags_raw == "" {
            println!("Empty")
        } else {
            let tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

            for emoji in tags.iter() {
                print!("{} ", emoji.tag);
            }
        }
    } else {
        println!("Empty");
    }

    Ok(())
}

pub fn remove_tag_by_tag(remove_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        if tags_raw == "" {
            println!("Empty")
        } else {
            let mut tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

            match utils::find_tag(&tags, remove_tag) {
                Some(index) => {
                    tags.remove(index);

                    let json_tag = serde_json::to_string(&tags)?;
                    utils::save(&json_tag, "emoji-tag.bin");
                }
                None => {
                    println!("Tag not found")
                }
            }
        }
    } else {
        println!("Not found database")
    }

    Ok(())
}

pub fn remove_all_tag() {
    utils::save(&"".to_string(), "emoji-tag.bin");
}

pub fn get_all() -> Result<(), Box<dyn std::error::Error>> {
    if utils::check_file_exist("emoji-tag.bin") {
        let tags_raw = utils::load("emoji-tag.bin");
        if tags_raw == "" {
            println!("empty")
        } else {
            let tags: Vec<model::EmojiTag> = serde_json::from_str(&tags_raw)?;

            for emoji in tags.iter() {
                println!("\n\n{}", emoji.tag);
                if emoji.emojis == "" {
                    println!("Empty");
                } else {
                    utils::print_list_emojis(&emoji.emojis);
                }
            }
        }
    } else {
        println!("Not found database")
    }

    Ok(())
}
