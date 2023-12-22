use client::{Finisterra, WINDOW_HEIGHT, WINDOW_WIDTH};
use engine::game::run_game;
use veril::Veril;

mod settings;

fn main() {
    let settings = engine::settings::Settings {
        width: WINDOW_WIDTH,
        height: WINDOW_HEIGHT,
        title: "Finisterra".to_string(),
        vsync: false,
    };
    // run_game(
    //     settings,
    //     || {
    //         add_font(
    //             "tahoma",
    //             include_bytes!("../../../assets/fonts/tahoma_bold.ttf").as_slice(),
    //         );
    //
    //         let ui = UI {
    //             main_texture: add_ui_texture(
    //                 include_bytes!("../../../assets/interface/main_ao_20_800x600_spells.png"),
    //                 TextureFilter::Nearest,
    //             ),
    //             border: 10,
    //             top_panel_size: 600 - 480 - 20,
    //             right_panel_size: 800 - 480 - 20,
    //         };
    //
    //         let mut finisterra = Finisterra::ao_20(ui);
    //         let textures_folder = "./assets/ao_20/graphics";
    //
    //         for image in finisterra.resources.images.iter().flatten() {
    //             let mut path = PathBuf::from(textures_folder);
    //             path.push(format!("{}.png", image.file_num));
    //
    //             let id = image.file_num;
    //             register_texture_with_id(path, id);
    //         }
    //
    //         const CHARS: usize = 15000;
    //         for i in 0..CHARS {
    //             let entity = Entity::random(1000000 + i * 10, &finisterra.resources);
    //
    //             let [x, y] = entity.position;
    //             finisterra.current_map.tiles[x as usize][y as usize].user = Some(i);
    //             finisterra.entities.push(entity);
    //         }
    //         finisterra
    //     },
    //     |game| game.game_loop(),
    //     |game, new_size| game.resize(new_size),
    // );

    run_game::<Finisterra, Veril>(settings);
}
