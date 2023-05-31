use std::collections::BTreeMap;

use crate::{
    animation::Animation, body::Body, fx::FX, gear::HeadGear, head::Head, image::Image, map::Map,
    shield::Shield, weapon::Weapon,
};

#[derive(Default, Clone, Debug)]
pub struct ClientResources {
    pub images: BTreeMap<usize, Image>,
    pub maps: BTreeMap<usize, Map>,
    pub animations: BTreeMap<usize, Animation>,
    pub bodies: BTreeMap<usize, Body>,
    pub heads: BTreeMap<usize, Head>,
    pub shields: BTreeMap<usize, Shield>,
    pub weapons: BTreeMap<usize, Weapon>,
    pub headgears: BTreeMap<usize, HeadGear>,
    pub fxs: BTreeMap<usize, FX>,
}

pub use crate::ao_99z::client::load_client_resources;
pub use crate::ao_99z::client::ClientResourcesPaths;
