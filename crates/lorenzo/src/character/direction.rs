use std::fmt::Display;

#[derive(Debug, Default, PartialEq, Copy, Clone)]
pub enum Direction {
    #[default]
    South,
    North,
    East,
    West,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Direction::South => "south",
            Direction::North => "north",
            Direction::East => "east",
            Direction::West => "west",
        })
    }
}
