use definitions::heading::Heading;

pub fn get_headgear_z(z: f32, _heading: Heading) -> f32 {
    z + 0.00001
}

pub fn get_weapon_z(z: f32, heading: Heading) -> f32 {
    match heading {
        Heading::South | Heading::East => z + 0.00001,
        Heading::North | Heading::West => z - 0.00001,
    }
}

pub fn get_shield_z(z: f32, heading: Heading) -> f32 {
    match heading {
        Heading::South | Heading::West => z + 0.00001,
        Heading::North | Heading::East => z - 0.00001,
    }
}
