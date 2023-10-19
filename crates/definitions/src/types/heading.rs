#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Heading {
    #[default]
    South = 0,
    North = 1,
    West = 2,
    East = 3,
}
