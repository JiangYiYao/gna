use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub category: u8,
    pub dimension: u8,
    pub dimension_data: Vec<usize>,
    pub content_data: Vec<u8>,
}

pub enum Signal {
    CheckClipboard,
    SendClipboardText,
    SendClipboardImage,
}