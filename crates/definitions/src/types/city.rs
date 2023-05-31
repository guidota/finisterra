use crate::map::WorldPosition;

#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct City {
    pub name: String,
    pub description: String,
    pub world_position: WorldPosition,
}
