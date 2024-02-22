use engine::engine::{GameEngine, TextureID};

pub const INPUT_ID: TextureID = 92000;
pub const BUTTON_ID: TextureID = 92001;
pub const WINDOW_ID: TextureID = 92002;
pub const DV_BACKGROUND_ID: TextureID = 92003;
pub const CHAR_SLOT_ID: TextureID = 92004;
pub const NEW_CHAR_SLOT_ID: TextureID = 92005;

pub struct Textures;

impl Textures {
    pub fn load<E: GameEngine>(engine: &mut E) {
        engine.set_texture("./assets/interface/input_field.png", INPUT_ID);
        engine.set_texture("./assets/interface/button.png", BUTTON_ID);
        engine.set_texture("./assets/interface/window.png", WINDOW_ID);
        engine.set_texture("./assets/interface/dv.png", DV_BACKGROUND_ID);
        engine.set_texture("./assets/interface/char_slot.png", CHAR_SLOT_ID);
        engine.set_texture("./assets/interface/char_create_slot.png", NEW_CHAR_SLOT_ID);
    }
}
