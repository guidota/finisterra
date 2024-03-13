use engine::{
    draw::{
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::GameEngine,
};

use super::{
    button::{Button, ButtonBuilder},
    colors::{GRAY_5, GRAY_6, RED, WHITE},
    fonts::TAHOMA_BOLD_8_SHADOW_ID,
    image::Image,
    textures::{LANZAR_BUTTON_ID, SPELLS_LIST_ID},
    Alignment, Widget,
};

const LIST_HEIGHT: u16 = 192;
const LINE_HEIGHT: u16 = 15;
const SPELLS_TO_SHOW: u16 = LIST_HEIGHT / LINE_HEIGHT;

const TOTAL_SPELLS: usize = 30;

pub struct Spellbook {
    background: Image,

    visible: bool,
    cast_button: Button,

    spell_offset: usize,
    spells: [Spell; TOTAL_SPELLS],

    pub position: (u16, u16), // top left
}

pub struct Spell {
    _spell_id: Option<u16>,
    text: ParsedText,
}

impl Spellbook {
    pub fn initialize<E: GameEngine>(engine: &mut E) -> Self {
        let background = Image::new(SPELLS_LIST_ID, WHITE, (0, 0));
        let cast_button = ButtonBuilder::new()
            .texture_id(LANZAR_BUTTON_ID)
            .selected_color(RED)
            .alignment(Alignment::Left)
            .color(GRAY_5)
            .z(1.)
            .build();

        let mut empty = || {
            let text = engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, "(Vacio)")
                .expect("can parse");

            Spell {
                _spell_id: None,
                text,
            }
        };
        let spells: [Spell; TOTAL_SPELLS] = [(); TOTAL_SPELLS].map(|_| empty());

        Self {
            background,
            position: (0, 0),
            visible: false,
            cast_button,
            spells,
            spell_offset: 0,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }
}

impl Spell {
    fn draw<E: engine::engine::GameEngine>(
        &mut self,
        context: &mut crate::game::Context<E>,
        x: u16,
        y: u16,
    ) {
        context.engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.text,
                position: Position::new(x, y, 1.),
                color: GRAY_6,
            },
            Target::UI,
        );
    }
}

impl Widget for Spellbook {
    fn update<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if !self.visible {
            return;
        }
        self.background.update(context);
        self.background.position = (
            self.position.0 + self.background.size.0 / 2,
            self.position.1 - self.background.size.1 / 2,
        );

        self.cast_button.update(context);
        self.cast_button.position = (
            self.position.0 - 14,
            self.position.1 - self.background.size.1 - self.cast_button.size.1,
        );
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if !self.visible {
            return;
        }
        self.background.draw(context);

        // draw spells
        let start = self.spell_offset;
        let end = self.spell_offset + SPELLS_TO_SHOW as usize;
        for i in start..end {
            let spell = &mut self.spells[i];
            spell.draw(
                context,
                self.position.0 + 22,
                self.position.1 + 5 - LINE_HEIGHT * (i as u16 + 1),
            );
        }

        self.cast_button.draw(context);
    }
}
