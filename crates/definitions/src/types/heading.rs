#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Heading {
    #[default]
    South,
    North,
    East,
    West,
}
