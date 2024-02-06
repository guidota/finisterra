#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Image {
    pub id: u32,

    /// file number corresponding to the image
    pub file: u64,

    /// source rect, sometimes the image is a portion of a bigger one
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
