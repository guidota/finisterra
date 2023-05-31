use crate::Offset;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct FX {
    pub animation: usize,
    pub offset: Offset,
}
