use engine::engine::{GameEngine, TextureID};

#[derive(Debug, Default)]
pub struct Textures {
    pub input: TextureID,
    pub button: TextureID,
    pub window: TextureID,
    pub dv_background: TextureID,
    pub char_slot: TextureID,
    pub char_create_slot: TextureID,
    pub main: TextureID,
    pub main_frame: TextureID,
    pub bar: TextureID,
    pub exp: TextureID,
    pub inventory_button: TextureID,
    pub inventory_button_disabled: TextureID,
    pub inventory_list: TextureID,
    pub lanzar_button: TextureID,
    pub spells_button: TextureID,
    pub spells_button_disabled: TextureID,
    pub spells_list: TextureID,
    pub spell_selection: TextureID,
    pub inventory_slot: TextureID,
    pub character_shadow: TextureID,
}

impl Textures {
    pub fn load<E: GameEngine>(engine: &mut E) -> Self {
        Self {
            input: engine.add_texture("./assets/interface/input_field.png"),
            button: engine.add_texture("./assets/interface/button.png"),
            window: engine.add_texture("./assets/interface/window.png"),
            dv_background: engine.add_texture("./assets/interface/dv.png"),
            char_slot: engine.add_texture("./assets/interface/char_slot.png"),
            char_create_slot: engine.add_texture("./assets/interface/char_create_slot.png"),
            main: engine.add_texture("./assets/interface/main/main.png"),
            main_frame: engine.add_texture("./assets/interface/main/main_frame.png"),
            bar: engine.add_texture("./assets/interface/main/bar.png"),
            exp: engine.add_texture("./assets/interface/main/exp.png"),
            inventory_button: engine.add_texture("./assets/interface/main/inventory_button.png"),
            inventory_button_disabled: engine
                .add_texture("./assets/interface/main/inventory_button_disabled.png"),
            inventory_list: engine.add_texture("./assets/interface/main/inventory_list.png"),
            lanzar_button: engine.add_texture("./assets/interface/main/lanzar_button.png"),
            spells_button: engine.add_texture("./assets/interface/main/spells_button.png"),
            spells_button_disabled: engine
                .add_texture("./assets/interface/main/spells_button_disabled.png"),
            spell_selection: engine.add_texture("./assets/interface/main/spell_selection.png"),
            spells_list: engine.add_texture("./assets/interface/main/spells_list.png"),
            inventory_slot: engine.add_texture("./assets/interface/main/inventory_slot.png"),
            character_shadow: engine.add_texture("./assets/finisterra/images/shadow-2.png"),
        }
    }
}
