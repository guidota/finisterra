#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Animation {
    pub id: usize,
    pub speed: usize,
    pub frames: Vec<u32>,
}
