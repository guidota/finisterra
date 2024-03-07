#![allow(dead_code)]
// Generated with sql-gen
//https://github.com/jayy-lmao/sql-gen

pub mod account_characters;
pub use account_characters::AccountCharacters;
pub mod account_characters_db_set;
pub use account_characters_db_set::AccountCharactersSet;

pub mod accounts;
pub use accounts::Accounts;
pub mod accounts_db_set;
pub use accounts_db_set::AccountsSet;

pub mod character_attributes;
pub use character_attributes::CharacterAttributes;
pub mod character_attributes_db_set;
pub use character_attributes_db_set::CharacterAttributesSet;

pub mod character_equipment;
pub use character_equipment::CharacterEquipment;
pub mod character_equipment_db_set;
pub use character_equipment_db_set::CharacterEquipmentSet;

pub mod character_inventories;
pub use character_inventories::CharacterInventories;
pub mod character_inventories_db_set;
pub use character_inventories_db_set::CharacterInventoriesSet;

pub mod character_skills;
pub use character_skills::CharacterSkills;
pub mod character_skills_db_set;
pub use character_skills_db_set::CharacterSkillsSet;

pub mod character_spellbooks;
pub use character_spellbooks::CharacterSpellbooks;
pub mod character_spellbooks_db_set;
pub use character_spellbooks_db_set::CharacterSpellbooksSet;

pub mod character_statistics;
pub use character_statistics::CharacterStatistics;
pub mod character_statistics_db_set;
pub use character_statistics_db_set::CharacterStatisticsSet;

pub mod character_vaults;
pub use character_vaults::CharacterVaults;
pub mod character_vaults_db_set;
pub use character_vaults_db_set::CharacterVaultsSet;

pub mod characters;
pub use characters::Characters;
pub mod characters_db_set;
pub use characters_db_set::CharactersSet;


pub struct PostgresContext;

impl PostgresContext {
  pub fn account_characters(&self) -> AccountCharactersSet { AccountCharactersSet }

  pub fn accounts(&self) -> AccountsSet { AccountsSet }

  pub fn character_attributes(&self) -> CharacterAttributesSet { CharacterAttributesSet }

  pub fn character_equipment(&self) -> CharacterEquipmentSet { CharacterEquipmentSet }

  pub fn character_inventories(&self) -> CharacterInventoriesSet { CharacterInventoriesSet }

  pub fn character_skills(&self) -> CharacterSkillsSet { CharacterSkillsSet }

  pub fn character_spellbooks(&self) -> CharacterSpellbooksSet { CharacterSpellbooksSet }

  pub fn character_statistics(&self) -> CharacterStatisticsSet { CharacterStatisticsSet }

  pub fn character_vaults(&self) -> CharacterVaultsSet { CharacterVaultsSet }

  pub fn characters(&self) -> CharactersSet { CharactersSet }

}