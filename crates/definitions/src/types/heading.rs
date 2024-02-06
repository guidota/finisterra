#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Heading {
    #[default]
    South = 0,
    North = 1,
    West = 2,
    East = 3,
}

impl Heading {
    pub fn iterator() -> impl Iterator<Item = &'static Heading> {
        static HEADINGS: [Heading; 4] =
            [Heading::South, Heading::North, Heading::West, Heading::East];
        HEADINGS.iter()
    }
}
