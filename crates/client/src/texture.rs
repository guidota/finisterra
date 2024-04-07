#[derive(Debug, Default)]
pub enum TextureState {
    #[default]
    Dirty,
    JustDraw,
    Ready,
}
