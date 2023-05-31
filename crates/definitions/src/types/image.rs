#[derive(Debug, Clone)]
pub struct Image {
    pub id: usize,
    pub file_num: u32,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
