use crate::model;
use crate::utils;

static FIRST_EMOJI: u32 = 127744;

pub fn add_emoji(add: &str, tag: &str) -> Result<(), Box<dyn std::error::Error>> {
    if utils::checkk_emoji(add) {
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
    } else {
        println!("Emoji not found")
    }

    Ok(())
}

pub fn remove_emoji_by_tag(
    remove_emoji: &str,
    tag: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    if utils::checkk_emoji(remove_emoji) {
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
    } else {
        println!("Emoji not found")
    }

    Ok(())
}

pub fn remove_all_emoji_by_tag(tag: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn get_tag(tag: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn create_tag(new_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn list_tab() -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn remove_tag_by_tag(remove_tag: &str) -> Result<(), Box<dyn std::error::Error>> {
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

    Ok(())
}

pub fn remove_all_tag() {
    utils::save(&"".to_string(), "emoji-tag.bin");
}
