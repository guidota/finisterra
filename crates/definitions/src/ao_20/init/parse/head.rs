use super::{get_count, get_number};
use crate::{ao_20::init::Head, parse::get_ini_reader};
use ini::Ini;
use std::collections::BTreeMap;

pub fn parse_heads_from_bytes(mut bytes: &[u8]) -> BTreeMap<usize, Head> {
    let mut heads = BTreeMap::new();

    let ini = Ini::read_from(&mut bytes).expect("File doesn't exist");
    for head in 1..=get_count(&ini, "NumHeads") {
        heads.insert(head, parse_head(head, &ini));
    }

    heads
}

pub fn parse_heads(path: &str) -> BTreeMap<usize, Head> {
    let mut heads = BTreeMap::new();

    let ini = get_ini_reader(path).expect("File doesn't exist");
    for head in 1..=get_count(&ini, "NumHeads") {
        heads.insert(head, parse_head(head, &ini));
    }

    heads
}

fn parse_head(head_number: usize, ini: &Ini) -> Head {
    let head_section = ini
        .section(Some(&format!("HEAD{head_number}")))
        .expect("Head {head_number} doesn't exist");
    Head(
        get_number(head_section, "Head1"),
        get_number(head_section, "Head2"),
        get_number(head_section, "Head3"),
        get_number(head_section, "Head4"),
    )
}
