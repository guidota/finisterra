use std::time::Duration;

use super::{Animation, ImageFrameMetadata};

pub type FXAnimation = Animation<ImageFrameMetadata>;

pub struct FX {
    pub animation: FXAnimation,

    pub loops: usize,
    pub time: Duration,
}
