use engine::engine::{GameEngine, TextureID};

pub const INPUT_ID: TextureID = 1;
pub const BUTTON_ID: TextureID = 2;
pub const WINDOW_ID: TextureID = 3;
pub const DV_BACKGROUND_ID: TextureID = 4;
pub const CHAR_SLOT_ID: TextureID = 5;
pub const NEW_CHAR_SLOT_ID: TextureID = 6;

// MAIN
pub const MAIN_UI_ID: TextureID = 7;
pub const BAR_ID: TextureID = 8;
pub const EXP_BAR_ID: TextureID = 9;
pub const INVENTORY_BUTTON_ID: TextureID = 10;
pub const INVENTORY_LIST_ID: TextureID = 11;
pub const LANZAR_BUTTON_ID: TextureID = 12;
pub const SPELLS_BUTTON_ID: TextureID = 13;
pub const SPELLS_LIST_ID: TextureID = 14;

pub struct Textures;

impl Textures {
    pub fn load<E: GameEngine>(engine: &mut E) {
        engine.set_texture("./assets/interface/input_field.png", INPUT_ID);
        engine.set_texture("./assets/interface/button.png", BUTTON_ID);
        engine.set_texture("./assets/interface/window.png", WINDOW_ID);
        engine.set_texture("./assets/interface/dv.png", DV_BACKGROUND_ID);
        engine.set_texture("./assets/interface/char_slot.png", CHAR_SLOT_ID);
        engine.set_texture("./assets/interface/char_create_slot.png", NEW_CHAR_SLOT_ID);
        // main UI components
        engine.set_texture("./assets/interface/main/main.png", MAIN_UI_ID);
        engine.set_texture("./assets/interface/main/bar.png", BAR_ID);
        engine.set_texture("./assets/interface/main/exp.png", EXP_BAR_ID);
        engine.set_texture(
            "./assets/interface/main/inventory_button.png",
            INVENTORY_BUTTON_ID,
        );
        engine.set_texture(
            "./assets/interface/main/inventory_list.png",
            INVENTORY_LIST_ID,
        );
        engine.set_texture(
            "./assets/interface/main/lanzar_button.png",
            LANZAR_BUTTON_ID,
        );
        engine.set_texture(
            "./assets/interface/main/spells_button.png",
            SPELLS_BUTTON_ID,
        );
        engine.set_texture("./assets/interface/main/spells_list.png", SPELLS_LIST_ID);
    }
}
