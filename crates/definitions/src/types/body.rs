use crate::Offset;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Body {
    pub animations: [usize; 4],
    pub head_offset: Offset,
}
