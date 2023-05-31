use crate::parse::Long;

pub mod parse;

#[derive(Debug, Clone)]
pub struct Image {
    pub file_num: Long,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub id: String,
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub frames: Vec<String>,
    pub speed: Long,
    pub id: String,
}
