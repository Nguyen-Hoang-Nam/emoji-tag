// use crate::model;
use std::env;

use crate::model;

static FIRST_EMOJI: u32 = 127744;

fn create_not_exist_path(path: &String) {
    if !std::path::Path::new(&path).exists() {
        std::fs::create_dir(path).expect("Can not create directory.");
    }
}

fn cache_path(file_name: &str) -> String {
    let os = env::consts::OS;
    let mut result = String::new();

    if os == "linux" {
        match env::var("XDG_DATA_HOME") {
            Ok(cache_directory) => {
                let path = format!("{}/emojitag", cache_directory);
                create_not_exist_path(&path);

                result = format!("{}/{}", path, file_name);
            }
            Err(..) => {
                let path = format!("$HOME/.emojitag");
                create_not_exist_path(&path);

                result = format!("{}/{}", path, file_name)
            }
        }
    } else if os == "windows" {
        let path = format!("%USERPROFILE\\AppData\\emojitag");
        create_not_exist_path(&path);

        result = format!("{}\\{}", path, file_name)
    } else if os == "macos" {
        let path = format!("~/Library/Caches/emojitag");
        create_not_exist_path(&path);

        result = format!("{}/{}", path, file_name)
    }

    result
}

pub fn check_file_exist(file_name: &str) -> bool {
    let file_path = cache_path(file_name);

    std::path::Path::new(&file_path).exists()
}

pub fn save(new_line: &String, file_name: &str) {
    let file_path = cache_path(file_name);

    savefile::prelude::save_file(&file_path, 0, new_line).unwrap();
}

pub fn load(file_name: &str) -> String {
    let file_path = cache_path(file_name);

    savefile::prelude::load_file(&file_path, 0).unwrap()
}

fn utf32_to_utf8(utf32: u32) -> Vec<u8> {
    let mut utf8 = vec![];

    if utf32 <= 0x7F {
        utf8.push(utf32 as u8);
        utf8.push(0);
        utf8.push(0);
        utf8.push(0);
    } else if utf32 <= 0x7FF {
        utf8.push((0xC0 | (utf32 >> 6)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
        utf8.push(0);
        utf8.push(0);
    } else if utf32 <= 0xFFFF {
        utf8.push((0xE0 | (utf32 >> 12)) as u8);
        utf8.push((0x80 | ((utf32 >> 6) & 0x3F)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
        utf8.push(0);
    } else if utf32 <= 0x10FFFF {
        utf8.push((0xF0 | (utf32 >> 18)) as u8);
        utf8.push((0x80 | ((utf32 >> 12) & 0x3F)) as u8);
        utf8.push((0x80 | ((utf32 >> 6) & 0x3F)) as u8);
        utf8.push((0x80 | (utf32 & 0x3F)) as u8);
    }

    utf8
}

fn from_utf32(utf32: u32) -> String {
    let utf8 = utf32_to_utf8(utf32);
    std::str::from_utf8(&utf8).unwrap().to_string()
}

// fn consecutive_emoji(from: u32, to: u32) {
//     let mut emoji_list: String = "".to_owned();
//     for emoji in from..to + 1 {
//         emoji_list.push_str(&from_utf32(emoji + FIRST_EMOJI));
//     }

//     print!("{}", emoji_list)
// }

pub fn print_list_emojis(emoji_raw: &String) {
    let mut emoji_list: String = "".to_owned();

    let emojis = emoji_raw.split(",");
    for emoji in emojis {
        let emoji_int: u32 = emoji.parse().unwrap();
        emoji_list.push_str(&from_utf32(emoji_int + FIRST_EMOJI));
    }

    print!("{}", emoji_list)
}

pub fn between(num: u32, min: u32, max: u32) -> bool {
    num >= min && num <= max
}

pub fn checkk_emoji(emoji_raw: &str) -> bool {
    if emoji_raw.len() == 0x4 {
        let utf32 = emoji_raw.chars().nth(0).unwrap() as u32;
        if between(utf32, 127744, 128591) {
            return true;
        }
    }

    return false;
}

pub fn find_tag(tags: &Vec<model::EmojiTag>, tag: &str) -> Option<usize> {
    let mut tag_index: Option<usize> = None;

    for (index, emoji) in tags.iter().enumerate() {
        if emoji.tag == tag {
            tag_index = Some(index);
            break;
        }
    }

    tag_index
}
