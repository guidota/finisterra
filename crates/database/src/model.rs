use sqlx::types::chrono::{DateTime, Utc};

pub struct CreateAccount<'s> {
    pub name: &'s str,
    pub email: &'s str,
    pub password: &'s str,
    pub pin: i32,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Account {
    pub name: String,
    pub email: String,
    pub password: String,
    pub pin: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug)]
pub struct AccountCharacters {
    pub id: i32,
    pub account_name: String,
    pub character_name: String,
}
#[derive(Clone)]
pub struct CreateCharacter {
    pub name: String,
    pub class_id: i32,
    pub race_id: i32,
    pub gender_id: i32,
    pub map: i32,
    pub x: i32,
    pub y: i32,

    pub attributes: Attributes,
    pub statistics: Statistics,
    pub look: Look,
    pub equipment: Equipment,
}

#[derive(sqlx::FromRow, Debug)]
pub struct CharacterPreview {
    pub name: String,
    pub description: String,
    pub level: i32,
    pub exp: i32,
    pub class_id: i32,
    pub race_id: i32,
    pub gender_id: i32,
    pub gold: i32,
    pub map: i32,
    pub x: i32,
    pub y: i32,

    #[sqlx(flatten)]
    pub look: Look,

    #[sqlx(flatten)]
    pub equipment: Equipment,
}

#[derive(sqlx::FromRow, Debug)]
pub struct Character {
    pub name: String,
    pub description: String,
    pub level: i32,
    pub exp: i32,
    pub class_id: i32,
    pub race_id: i32,
    pub gender_id: i32,

    pub gold: i64,
    pub map: i32,
    pub x: i32,
    pub y: i32,

    pub inventory: Vec<u8>,
    pub spellbook: Vec<u8>,
    pub vault: Vec<u8>,
    pub skills: Vec<u8>,

    #[sqlx(flatten)]
    pub stats: Statistics,

    #[sqlx(flatten)]
    pub attributes: Attributes,

    #[sqlx(flatten)]
    pub look: Look,

    #[sqlx(flatten)]
    pub equipment: Equipment,

    pub created_at: DateTime<Utc>,
}

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Attributes {
    pub strength: i32,
    pub agility: i32,
    pub intelligence: i32,
    pub charisma: i32,
    pub constitution: i32,
}

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Look {
    pub body: i32,
    pub face: i32,
    pub skin: i32,
    pub hair: i32,
}

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Equipment {
    pub weapon: Option<i32>,
    pub shield: Option<i32>,
    pub headgear: Option<i32>,
    pub clothing: Option<i32>,
}

#[derive(sqlx::FromRow, Debug, Clone, Default)]
pub struct Statistics {
    pub health: i32,
    pub mana: i32,
    pub stamina: i32,
    pub max_health: i32,
    pub max_mana: i32,
    pub max_stamina: i32,
}
