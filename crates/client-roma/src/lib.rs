use ao::{ao_20::maps::parse::parse_map, Map};
use resources::Resources;
use roma_wgpu::{
    graphics::{rect::Rect, textures::DrawTextureParams, vec2::vec2},
    Color, Roma,
};
use settings::Settings;

pub mod resources;
pub mod settings;

pub struct App {
    pub settings: Settings,
    pub resources: Resources,
    pub current_map: Map,
}

impl Default for App {
    fn default() -> Self {
        let current_map = parse_map("./assets/maps", 1).expect("can parse map");
        Self {
            settings: Settings::default(),
            resources: Resources::load(),
            current_map,
        }
    }
}

const TILE_SIZE: usize = 32;

impl App {
    pub fn update_camera(&self, _roma: &mut Roma) {
        // roma.graphics.set_camera_position(vec2(0. * 32., 0. * 32.))
    }

    pub fn draw_map(&self, roma: &mut Roma) {
        for i in 1..4 {
            for y in 0..30 {
                for x in 0..30 {
                    let tile = self.current_map.tiles[x + 50][99 - y - 50];
                    if tile.graphics[i] != 0 {
                        self.draw_image(
                            roma,
                            format!("{x}-{y}-{i}"),
                            tile.graphics[i] as usize,
                            (x * TILE_SIZE) as f32,
                            (y * TILE_SIZE) as f32,
                        );
                    }
                }
            }
        }
    }

    fn draw_image(&self, roma: &mut Roma, entity_id: String, image_id: usize, x: f32, y: f32) {
        if let Some(image) = self.resources.images.get(&image_id.to_string()) {
            let texture_id = image.file_num.to_string();
            let image_path = format!("./assets/graphics/{texture_id}.png");
            roma.graphics.load_texture(texture_id.clone(), &image_path);
            roma.graphics.draw_texture(
                entity_id,
                texture_id,
                x,
                y,
                Color::WHITE,
                Some(DrawTextureParams {
                    source: Some(Rect::new(
                        image.x as f32,
                        image.y as f32,
                        image.width as f32,
                        image.height as f32,
                    )),
                    flip_y: true,
                    ..Default::default()
                }),
            );
        }
    }
}
