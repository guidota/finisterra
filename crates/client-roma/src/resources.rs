use std::collections::BTreeMap;

use ao::ao_20::{
    graphics::{parse::parse_graphics, Animation, Image},
    init::{
        parse::{
            body::parse_bodies,
            head::parse_heads,
            template::{parse_templates, Template},
            weapon::parse_weapons,
        },
        Body, Head, Weapon,
    },
};

pub struct Resources {
    pub images: BTreeMap<String, Image>,
    pub animations: BTreeMap<String, Animation>,
    pub bodies: BTreeMap<usize, Body>,
    pub heads: BTreeMap<usize, Head>,
    pub weapons: BTreeMap<usize, Weapon>,
    pub body_templates: BTreeMap<usize, Template>,
}

impl Resources {
    pub fn load() -> Self {
        let (images, animations) =
            parse_graphics("./assets/init/graficos.ind").expect("can parse graphics");
        let bodies = parse_bodies("./assets/init/cuerpos.dat").expect("can parse bodies");
        let heads = parse_heads("./assets/init/cabezas.ini");
        let weapons = parse_weapons("./assets/init/armas.dat").expect("can parse weapons");
        let body_templates = parse_templates("./assets/init/moldes.ini");

        Resources {
            images,
            animations,
            bodies,
            heads,
            weapons,
            body_templates,
        }
    }
}
