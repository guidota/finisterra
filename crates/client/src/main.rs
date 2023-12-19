use client::{entity::Entity, ui::UI, Finisterra, WINDOW_HEIGHT, WINDOW_WIDTH};
use roma::{add_font, add_ui_texture, run_game, ui::paint::TextureFilter, Settings};

mod settings;

fn main() {
    let base_path = "./assets/ao_99z/graphics/".to_string();
    let settings = Settings {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: "Finisterra".to_string(),
        present_mode: roma::PresentMode::AutoNoVsync,
        // textures_folder: base_path,
    };
    run_game(
        settings,
        || {
            add_font(
                "tahoma",
                include_bytes!("../../../assets/fonts/tahoma_bold.ttf").as_slice(),
            );

            let ui = UI {
                main_texture: add_ui_texture(
                    include_bytes!("../../../assets/interface/main_ao_20_800x600_spells.png"),
                    TextureFilter::Nearest,
                ),
                border: 10,
                top_panel_size: 600 - 480 - 20,
                right_panel_size: 800 - 480 - 20,
            };

            let mut finisterra = Finisterra::ao_99z(ui);

            // let mut entity = Entity::random(1, &finisterra.resources);
            // let position = [1., 0.];
            // entity.position = position;
            // entity.world_position = [(position[0] * 32.) + 16., position[1] * 32.];
            // entity.state.direction = Heading::South;
            // finisterra.entities.push(entity);
            // finisterra.current_map.tiles[position[0] as usize][position[1] as usize].user = Some(0);
            //
            // let mut entity = Entity::random(2, &finisterra.resources);
            // let position = [0., 1.];
            // entity.position = position;
            // entity.world_position = [(position[0] * 32.) + 16., position[1] * 32.];
            // entity.state.direction = Heading::South;
            // finisterra.entities.push(entity);
            // finisterra.current_map.tiles[position[0] as usize][position[1] as usize].user = Some(1);
            const CHARS: usize = 15000;
            for i in 0..CHARS {
                let entity = Entity::random(1000000 + i * 10, &finisterra.resources);

                let [x, y] = entity.position;
                finisterra.current_map.tiles[x as usize][y as usize].user = Some(i);
                finisterra.entities.push(entity);
            }
            finisterra
        },
        |game| game.game_loop(),
        |game, new_size| game.resize(new_size),
    );
}
