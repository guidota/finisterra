#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Head {
    /// Static images
    pub images: [usize; 4],
}
