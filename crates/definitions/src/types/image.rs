#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Image {
    pub id: u32,
    pub file_num: u64,
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
