use engine::draw::Color;

pub const BLACK: Color = [0, 0, 0, 255];
pub const WHITE: Color = [255, 255, 255, 255];

pub const GRAY_1: Color = [40, 40, 40, 255];
pub const GRAY_2: Color = [79, 79, 79, 255];
pub const GRAY_3: Color = [130, 130, 130, 255];
pub const GRAY_4: Color = [189, 189, 189, 255];
pub const GRAY_5: Color = [224, 224, 224, 255];
pub const GRAY_6: Color = [242, 242, 242, 255];

pub const RED: Color = [235, 87, 87, 255];
pub const ORANGE: Color = [242, 153, 74, 255];
pub const YELLOW: Color = [242, 201, 76, 255];

pub const GREEN: Color = [33, 150, 83, 255];
pub const GREEN_2: Color = [39, 174, 96, 255];
pub const GREEN_3: Color = [111, 207, 151, 255];

pub const BLUE: Color = [47, 128, 237, 255];
pub const BLUE_2: Color = [45, 156, 219, 255];
pub const BLUE_3: Color = [86, 204, 242, 255];

pub const PURPLE: Color = [155, 81, 224, 255];
pub const PURPLE_2: Color = [187, 107, 217, 255];

pub fn tint(color: Color, percent: f32) -> Color {
    let mut to_color = color;
    for i in 0..3 {
        to_color[i] = std::cmp::min(
            255,
            (color[i] as f32 + ((255. - color[i] as f32) * percent)).round() as u8,
        );
    }
    to_color
}

pub fn shade(color: Color, factor: f32) -> Color {
    let mut to_color = color;
    for i in 0..=3 {
        to_color[i] = (color[i] as f32 * factor).round() as u8;
    }
    to_color
}
