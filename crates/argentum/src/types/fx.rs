use crate::Offset;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct FX {
    pub animation: usize,
    pub offset: Offset,
}
