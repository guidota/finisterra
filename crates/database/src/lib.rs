use std::env;

use anyhow::Result;
use model::{Account, Character, CharacterPreview, CreateAccount, CreateCharacter};
use sqlx::{
    migrate::MigrateDatabase,
    types::chrono::{DateTime, Utc},
    Acquire, SqlitePool,
};

pub mod model;
pub mod types;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    pub async fn initialize() -> Result<Self> {
        let db_url = env::var("DATABASE_URL").unwrap_or("sqlite:argentum.db".to_string());
        if !sqlx::Sqlite::database_exists(&db_url).await? {
            sqlx::Sqlite::create_database(&db_url).await?;
        }

        let pool = SqlitePool::connect(&db_url).await?;
        sqlx::migrate!().run(&pool).await?;

        Ok(Self { pool })
    }

    pub async fn create_account(&self, create_account: &CreateAccount<'_>) -> Result<Account> {
        let mut conn = self.pool.acquire().await?;

        let account = sqlx::query_as::<_, Account>(r#"INSERT INTO "accounts" ("name", "email", "password", "pin") VALUES ($1, $2, $3, $4) RETURNING *;"#)
            .bind(create_account.name)
            .bind(create_account.email)
            .bind(create_account.password)
            .bind(create_account.pin)
            .fetch_one(&mut *conn)
            .await?;

        Ok(account)
    }

    pub async fn account(&self, mail: &str) -> Result<Account> {
        let mut conn = self.pool.acquire().await?;
        let account = sqlx::query_as::<_, Account>("SELECT * FROM accounts as a WHERE a.name = $1")
            .bind(mail)
            .fetch_one(&mut *conn)
            .await?;

        Ok(account)
    }

    pub async fn account_characters(&self, account_name: &str) -> Result<Vec<CharacterPreview>> {
        let mut conn = self.pool.acquire().await?;
        let characters = sqlx::query_as::<_, CharacterPreview>(
            r#"              
            SELECT char.*,
                look.body, look.face, look.skin, look.hair,
                equipment.weapon, equipment.shield, equipment.headgear, equipment.clothing
            FROM characters AS char 

            JOIN character_equipment as equipment ON char.name = equipment.name
            JOIN character_look as look ON char.name = look.name

            WHERE char.name IN (SELECT character_name FROM account_characters WHERE account_name = $1);
            "#,
        )
        .bind(account_name)
        .fetch_all(&mut *conn)
        .await;

        characters.map_err(|e| {
            println!("error while querying characters {e}");
            anyhow::anyhow!("{e}")
        })
    }

    pub async fn character(&self, character_name: &str) -> Result<Character> {
        let mut conn = self.pool.acquire().await?;
        let character = sqlx::query_as::<_, Character>(
            r#"              
            SELECT 
                char.*,
                stats.health, stats.mana, stats.stamina, stats.max_health, stats.max_mana, stats.max_stamina,
                attributes.strength, attributes.agility, attributes.intelligence, attributes.charisma, attributes.constitution,
                look.body, look.face, look.skin, look.hair,
                equipment.weapon, equipment.shield, equipment.headgear, equipment.clothing,
                character_skills.value as skills,
                character_inventory.value as inventory,
                character_vault.value as vault,
                character_spellbook.value as spellbook 
            FROM characters AS char 

            JOIN character_statistics as stats ON char.name = stats.name
            JOIN character_attributes as attributes ON char.name = attributes.name
            JOIN character_equipment as equipment ON char.name = equipment.name
            JOIN character_skills ON char.name = character_skills.name
            JOIN character_inventories as character_inventory ON char.name = character_inventory.name
            JOIN character_vaults as character_vault ON char.name = character_vault.name
            JOIN character_spellbooks as character_spellbook ON char.name = character_spellbook.name
            JOIN character_look as look ON char.name = look.name

            WHERE char.name = $1
            "#,
        )
        .bind(character_name)
        .fetch_one(&mut *conn)
        .await?;

        Ok(character)
    }
    pub async fn insert_character(
        &self,
        account_name: &str,
        character: CreateCharacter,
    ) -> Result<Character> {
        let mut conn = self.pool.acquire().await?;

        let mut transaction = conn.begin().await?;

        let CreateCharacter {
            attributes,
            statistics,
            look,
            equipment,
            ..
        } = &character;

        sqlx::query(r#"INSERT INTO "characters" ("name", "class_id", "race_id", "gender_id", "map", "x", "y") VALUES ($1, $2, $3, $4, $5, $6, $7)"#)
            .bind(&character.name)
            .bind(character.class_id)
            .bind(character.race_id)
            .bind(character.gender_id)
            .bind(character.map)
            .bind(character.x)
            .bind(character.y)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_inventories" ("name", "value") VALUES ($1, $2)"#)
            .bind(&character.name)
            .bind(vec![])
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_attributes" ("name", "strength", "agility", "intelligence", "charisma", "constitution") VALUES ($1, $2, $3, $4, $5, $6)"#)
            .bind(&character.name)
            .bind(attributes.strength)
            .bind(attributes.agility)
            .bind(attributes.intelligence)
            .bind(attributes.charisma)
            .bind(attributes.constitution)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_statistics" ("name", "health", "mana", "stamina", "max_health", "max_mana", "max_stamina") VALUES ($1, $2, $3, $4, $5, $6, $7)"#)
            .bind(&character.name)
            .bind(statistics.health)
            .bind(statistics.mana)
            .bind(statistics.stamina)
            .bind(statistics.max_health)
            .bind(statistics.max_mana)
            .bind(statistics.max_stamina)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_look" ("name", "body", "face", "skin", "hair") VALUES ($1, $2, $3, $4, $5)"#)
            .bind(&character.name)
            .bind(look.body)
            .bind(look.face)
            .bind(look.skin)
            .bind(look.hair)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_equipment" ("name", "weapon", "shield", "clothing", "headgear") VALUES ($1, $2, $3, $4, $5)"#)
            .bind(&character.name)
            .bind(equipment.weapon)
            .bind(equipment.shield)
            .bind(equipment.clothing)
            .bind(equipment.headgear)
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_vaults" ("name", "value") VALUES ($1, $2)"#)
            .bind(&character.name)
            .bind(vec![])
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_skills" ("name", "value") VALUES ($1, $2)"#)
            .bind(&character.name)
            .bind(vec![])
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "character_spellbooks" ("name", "value") VALUES ($1, $2)"#)
            .bind(&character.name)
            .bind(vec![])
            .execute(&mut *transaction)
            .await?;

        sqlx::query(r#"INSERT INTO "account_characters" ("account_name", "character_name") VALUES ($1, $2)"#)
            .bind(account_name)
            .bind(&character.name)
            .execute(&mut *transaction)
            .await?;

        transaction.commit().await?;

        Ok(Character {
            name: character.name.to_string(),
            description: String::new(),
            level: 1,
            exp: 0,
            class_id: character.class_id,
            race_id: character.race_id,
            gender_id: character.gender_id,
            created_at: DateTime::<Utc>::MIN_UTC,
            inventory: vec![],
            spellbook: vec![],
            vault: vec![],
            skills: vec![],
            attributes: attributes.clone(),
            equipment: equipment.clone(),
            look: look.clone(),
            gold: 0,
            map: character.map,
            x: character.x,
            y: character.y,
            stats: statistics.clone(),
        })
    }
}
