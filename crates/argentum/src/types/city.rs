use crate::map::WorldPosition;

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct City {
    pub name: String,
    pub description: String,
    pub world_position: WorldPosition,
}
