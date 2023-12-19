use serde_with::serde_as;
use serde_with::DurationMilliSeconds;
use std::time::Duration;

use crate::Offset;

pub mod fx;

#[serde_as]
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct Animation<T> {
    pub frames: Vec<T>,
    #[serde_as(as = "DurationMilliSeconds")]
    pub duration: Duration,
}

#[derive(Debug, Default, Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct ImageFrameMetadata {
    pub image: u32,
    pub priority: u32,
    pub offset: Offset,
}
