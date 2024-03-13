use engine::engine::{FontID, GameEngine};
use tracing::info;

pub const TAHOMA_BOLD_8_SHADOW_ID: FontID = 90000;
pub const TAHOMA_REGULAR_8_ID: FontID = 90001;
pub const TAHOMA_BOLD_8_ID: FontID = 90002;
pub const TAHOMA_BOLD_10_ID: FontID = 90003;
pub const TAHOMA_REGULAR_10_ID: FontID = 90004;

pub const WIZARD_16_ID: FontID = 90010;
pub const WIZARD_24_ID: FontID = 90011;
pub const WIZARD_30_ID: FontID = 90012;

pub struct Fonts;

impl Fonts {
    pub fn load<E: GameEngine>(engine: &mut E) {
        let texture_id = engine.add_texture("./assets/fonts/tahoma_bold_8_shadow.png");
        info!("texture_id {texture_id}");
        engine.add_font(
            TAHOMA_BOLD_8_SHADOW_ID,
            "./assets/fonts/tahoma_bold_8.fnt",
            texture_id,
        );
        let texture_id = engine.add_texture("./assets/fonts/tahoma_bold_8.png");
        engine.add_font(
            TAHOMA_BOLD_8_ID,
            "./assets/fonts/tahoma_bold_8.fnt",
            texture_id,
        );
        let texture_id = engine.add_texture("./assets/fonts/tahoma_bold_10.png");
        engine.add_font(
            TAHOMA_BOLD_10_ID,
            "./assets/fonts/tahoma_bold_10.fnt",
            texture_id,
        );

        let texture_id = engine.add_texture("./assets/fonts/tahoma_regular_8.png");
        engine.add_font(
            TAHOMA_REGULAR_8_ID,
            "./assets/fonts/tahoma_regular_8.fnt",
            texture_id,
        );

        let texture_id = engine.add_texture("./assets/fonts/tahoma_regular_10.png");
        engine.add_font(
            TAHOMA_REGULAR_10_ID,
            "./assets/fonts/tahoma_regular_10.fnt",
            texture_id,
        );

        let texture_id = engine.add_texture("./assets/fonts/wizard's_manse_regular_16_shadow.png");
        engine.add_font(
            WIZARD_16_ID,
            "./assets/fonts/wizard's_manse_regular_16.fnt",
            texture_id,
        );

        let texture_id = engine.add_texture("./assets/fonts/wizard's_manse_regular_24.png");
        engine.add_font(
            WIZARD_24_ID,
            "./assets/fonts/wizard's_manse_regular_24.fnt",
            texture_id,
        );

        let texture_id = engine.add_texture("./assets/fonts/wizard's_manse_regular_30.png");
        engine.add_font(
            WIZARD_30_ID,
            "./assets/fonts/wizard's_manse_regular_30.fnt",
            texture_id,
        );
    }
}
