use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct EmojiTag {
    pub tag: String,
    pub emojis: String,
}
