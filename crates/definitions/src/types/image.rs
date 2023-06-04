#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Image {
    pub id: usize,
    pub file_num: usize,
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}
