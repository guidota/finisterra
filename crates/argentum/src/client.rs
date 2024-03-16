use std::rc::Rc;

use rustc_hash::FxHashMap;

use crate::{
    body::Body, fx::FX, gear::HeadGear, head::Head, image::Image, map::Map, shield::Shield,
    weapon::Weapon,
};

#[derive(Default, Clone, Debug)]
pub struct ClientResources {
    pub images: Vec<Option<Rc<Image>>>,
    pub maps: FxHashMap<usize, Map>,
    // pub animations: Vec<Option<Rc<Animation>>>,
    pub bodies: FxHashMap<usize, Body>,
    pub heads: FxHashMap<usize, Head>,
    pub shields: FxHashMap<usize, Shield>,
    pub weapons: FxHashMap<usize, Weapon>,
    pub headgears: FxHashMap<usize, HeadGear>,
    pub fxs: FxHashMap<usize, FX>,
}
