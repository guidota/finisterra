use std::fs;

use lorenzo::{
    animations::ImageFrameMetadata,
    character::{
        animation::{Animations, BodyAnimation},
        Body, BodyFrameMetadata, Skin,
    },
    Offset,
};

/// Generate metadata file samples
fn main() {
    let body = Body {
        idle: body_animations(),
        walk: body_animations(),
        attack: None,
        defend: None,
        die: None,
    };
    let pretty_config = ron::ser::PrettyConfig::new()
        .indentor(String::from("  "))
        .depth_limit(4)
        // .compact_arrays(true)
        .separate_tuple_members(true);
    let body_ron = ron::ser::to_string_pretty(&body, pretty_config.clone()).unwrap();
    fs::write("crates/lorenzo/resources/output/body.ron", body_ron).expect("Unable to write file");

    let skin = Skin {
        idle: skin_animations(),
        walk: skin_animations(),
        attack: None,
        defend: None,
        die: None,
    };
    let skin_ron = ron::ser::to_string_pretty(&skin, pretty_config).unwrap();
    fs::write("crates/lorenzo/resources/output/skin.ron", skin_ron).expect("Unable to write file");
}

fn body_animations() -> Animations<BodyFrameMetadata> {
    Animations {
        south: static_animation(),
        north: static_animation(),
        east: static_animation(),
        west: static_animation(),
    }
}

fn static_animation() -> BodyAnimation<BodyFrameMetadata> {
    BodyAnimation {
        frames: vec![body_frame()],
    }
}

fn body_frame() -> BodyFrameMetadata {
    BodyFrameMetadata {
        base: Offset::ZERO,
        head: Offset::ZERO,
        left_hand: Offset::ZERO,
        right_hand: Offset::ZERO,
        left_foot: Offset::ZERO,
        right_foot: Offset::ZERO,
    }
}

fn skin_animations() -> Animations<ImageFrameMetadata> {
    Animations {
        south: skin_static_animation(),
        north: skin_static_animation(),
        east: skin_static_animation(),
        west: skin_static_animation(),
    }
}

fn skin_static_animation() -> BodyAnimation<ImageFrameMetadata> {
    BodyAnimation {
        frames: vec![skin_frame()],
    }
}

fn skin_frame() -> ImageFrameMetadata {
    ImageFrameMetadata {
        image: 0,
        priority: 0,
        offset: Offset { x: 0, y: 0 },
    }
}
