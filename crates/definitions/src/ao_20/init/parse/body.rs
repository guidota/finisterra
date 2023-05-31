use super::{get_count, get_number, to_number};
use crate::{ao_20::init::Body, parse::get_ini_reader};
use ini::Ini;
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum ParseError {
    FileNotFound,
    NoBodyFound(usize),
    StaticWithoutStd,
    AnimatedWithoutWalks,
    BadHeadOffset,
}

pub fn parse_bodies_from_bytes(mut bytes: &[u8]) -> Result<BTreeMap<usize, Body>, ParseError> {
    let mut bodies = BTreeMap::new();

    let ini = Ini::read_from(&mut bytes).expect("Can read from bytes");
    for body_number in 1..=get_count(&ini, "NumBodies") {
        let body = parse_body(body_number, &ini);

        match body {
            Err(ParseError::NoBodyFound(number)) => {
                println!("Body {number} not found, continue.");
                continue;
            }
            Err(e) => {
                return Err(e);
            }
            Ok(body) => {
                bodies.insert(body_number, body);
            }
        }
    }

    Ok(bodies)
}

pub fn parse_bodies(path: &str) -> Result<BTreeMap<usize, Body>, ParseError> {
    let mut bodies = BTreeMap::new();

    let ini = get_ini_reader(path).expect("File doesn't exist");

    for body_number in 1..=get_count(&ini, "NumBodies") {
        let body = parse_body(body_number, &ini);

        match body {
            Err(ParseError::NoBodyFound(number)) => {
                println!("Body {number} not found, continue.");
                continue;
            }
            Err(e) => {
                return Err(e);
            }
            Ok(body) => {
                bodies.insert(body_number, body);
            }
        }
    }

    Ok(bodies)
}

fn parse_body(body_number: usize, ini: &Ini) -> Result<Body, ParseError> {
    let body_section = ini
        .section(Some(&format!("BODY{body_number}")))
        .ok_or(ParseError::NoBodyFound(body_number))?;

    let head_offset = (
        body_section
            .get("HeadOffsetX")
            .unwrap_or("0")
            .parse::<isize>()
            .map_err(|_| ParseError::BadHeadOffset)?,
        body_section
            .get("HeadOffsetY")
            .unwrap_or("0")
            .parse::<isize>()
            .map_err(|_| ParseError::BadHeadOffset)?,
    );

    let file_num = body_section.get("FileNum").map(to_number);
    if let Some(file_num) = file_num {
        let std = get_number(body_section, "Std");
        return Ok(Body::AnimatedWithTemplate {
            template_id: std,
            file_num,
            head_offset,
        });
    }

    Ok(Body::Animated {
        walks: (
            get_number(body_section, "Walk1"),
            get_number(body_section, "Walk2"),
            get_number(body_section, "Walk3"),
            get_number(body_section, "Walk4"),
        ),
        head_offset,
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_bodies() {}
}
