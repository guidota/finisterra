use std::fmt::Display;

#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Race {
    #[default]
    Human,
    Elf,
    Dwarf,
    Gnome,
    Drow,
}

impl Display for Race {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Race::Human => "Humano",
            Race::Elf => "Elfo",
            Race::Dwarf => "Enano",
            Race::Gnome => "Gnomo",
            Race::Drow => "Elfo Oscuro",
        })
    }
}
