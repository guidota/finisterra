#[derive(Debug, Clone)]
pub struct Animation {
    pub id: usize,
    pub speed: usize,
    pub frames: Vec<usize>,
}
