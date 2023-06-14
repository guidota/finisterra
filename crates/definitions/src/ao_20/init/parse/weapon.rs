use super::{get_count, get_number};
use crate::{parse::get_ini_reader, weapon::Weapon};
use ini::Ini;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum ParseError {
    FileNotFound,
    WeaponNotFound(usize),
}

pub fn parse_weapons_from_bytes(mut bytes: &[u8]) -> Result<BTreeMap<usize, Weapon>, ParseError> {
    let mut map = BTreeMap::new();

    let ini = Ini::read_from(&mut bytes).expect("Can parse bytes");
    for number in 1..=get_count(&ini, "NumArmas") {
        if let Ok(weapon) = parse_weapon(number, &ini) {
            map.insert(number, weapon);
        }
    }

    Ok(map)
}

pub fn parse_weapons(path: &str) -> Result<BTreeMap<usize, Weapon>, ParseError> {
    let mut map = BTreeMap::new();

    let ini = get_ini_reader(path).map_err(|_| ParseError::FileNotFound)?;
    for number in 1..=get_count(&ini, "NumArmas") {
        if let Ok(weapon) = parse_weapon(number, &ini) {
            map.insert(number, weapon);
        }
    }

    Ok(map)
}

fn parse_weapon(number: usize, ini: &Ini) -> Result<Weapon, ParseError> {
    let section = ini
        .section(Some(&format!("Arma{number}")))
        .ok_or(ParseError::WeaponNotFound(number))?;
    Ok(Weapon {
        animations: [
            get_number(section, "Dir1"),
            get_number(section, "Dir2"),
            get_number(section, "Dir3"),
            get_number(section, "Dir4"),
        ],
    })
}
