#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct HeadGear {
    /// Static images
    pub images: [usize; 4],
}
