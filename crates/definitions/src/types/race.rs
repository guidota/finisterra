#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub enum Race {
    #[default]
    Human,
    Elf,
    Dwarf,
    Gnome,
    Drow,
}
