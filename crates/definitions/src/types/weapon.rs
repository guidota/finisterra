#[derive(Default, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct Weapon {
    pub animations: [usize; 4],
}
