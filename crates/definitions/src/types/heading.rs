#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub enum Heading {
    #[default]
    South,
    North,
    East,
    West,
}
