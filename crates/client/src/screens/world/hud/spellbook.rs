use std::{ops::AddAssign, time::Duration};

use engine::{
    draw::{
        image::DrawImage,
        text::{DrawText, ParsedText},
        Position, Target,
    },
    engine::GameEngine,
    input::mouse,
};

use crate::game::Context;

use crate::ui::button::{Button, ButtonBuilder};
use crate::ui::colors::*;
use crate::ui::fonts::*;
use crate::ui::texture::Texture;
use crate::ui::{Alignment, Widget};

const LIST_HEIGHT: u16 = 195;
const LINE_HEIGHT: u16 = 13;
const SPELLS_TO_SHOW: u16 = LIST_HEIGHT / LINE_HEIGHT;

const TOTAL_SPELLS: usize = 30;

pub struct Spellbook {
    background: Texture,

    visible: bool,
    cast_button: Button,

    offset: usize,
    selection: usize,
    spells: [Spell; TOTAL_SPELLS],

    mouse_dragging: bool,
    mouse_dragging_timer: Duration,

    pub position: (u16, u16), // top left
}

pub struct Spell {
    _spell_id: Option<u16>,
    text: ParsedText,
}

impl Spellbook {
    pub fn initialize<E: GameEngine>(context: &mut Context<E>) -> Self {
        let background = Texture::new(context.resources.textures.spells_list, WHITE, (0, 0));
        let cast_button = ButtonBuilder::new()
            .texture_id(context.resources.textures.lanzar_button)
            .alignment(Alignment::Left)
            .color(GRAY_5)
            .z(1.)
            .target(Target::UI)
            .build();

        let mut i = 0;
        let mut empty = || {
            let text = context
                .engine
                .parse_text(TAHOMA_BOLD_8_SHADOW_ID, &format!("(Vacio) {i}"))
                .expect("can parse");
            i += 1;

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
            offset: 0,
            selection: 0,
            mouse_dragging: false,
            mouse_dragging_timer: Duration::ZERO,
        }
    }

    pub fn show(&mut self) {
        self.visible = true;
    }

    pub fn hide(&mut self) {
        self.visible = false;
    }

    pub fn is_dragging(&self) -> bool {
        self.visible && self.mouse_dragging
    }
}

impl Spell {
    fn draw<E: engine::engine::GameEngine>(
        &mut self,
        context: &mut crate::game::Context<E>,
        x: u16,
        y: u16,
    ) {
        let x = x + self.text.total_width / 2;
        context.engine.draw_text(
            TAHOMA_BOLD_8_SHADOW_ID,
            DrawText {
                text: &self.text,
                position: Position::new(x, y, 1.),
                color: GRAY_4,
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
            self.position.0 - 4,
            self.position.1 - self.background.size.1 - self.cast_button.size.1,
        );

        let mouse_position = || {
            let mouse::Position { x, y } = context.engine.mouse_position();
            let zoom = context.engine.get_camera_zoom();
            match zoom {
                engine::camera::Zoom::None => (x, y),
                engine::camera::Zoom::Double => (x / 2., y / 2.),
            }
        };

        let list_y_start = (self.position.1 - 7) as f32;
        let list_y_end = (self.position.1 - 7 - LIST_HEIGHT) as f32;
        // if mouse clicked or held and inside spell list
        if context.engine.mouse_clicked() {
            let (x, y) = mouse_position();
            if x > self.position.0 as f32
                && x < (self.position.0 + self.background.size.0) as f32
                && y < list_y_start
                && y > list_y_end
            {
                self.mouse_dragging = true;
                self.mouse_dragging_timer = Duration::ZERO;
                let y = list_y_start as u16 - y as u16;
                let selection = y / LINE_HEIGHT;
                self.selection = self.offset + selection as usize;
            }
        }
        if self.mouse_dragging {
            if context.engine.mouse_held() {
                let (_, y) = mouse_position();
                self.mouse_dragging_timer
                    .add_assign(context.engine.get_delta());
                if self.mouse_dragging_timer > Duration::from_millis(10) {
                    self.mouse_dragging_timer = Duration::ZERO;
                    if y > list_y_start {
                        self.offset = if self.offset == 0 { 0 } else { self.offset - 1 };
                        self.selection = self.offset;
                    } else if y < list_y_end {
                        self.offset =
                            std::cmp::min(TOTAL_SPELLS - SPELLS_TO_SHOW as usize, self.offset + 1);
                        self.selection = std::cmp::min(TOTAL_SPELLS - 1, self.selection + 1);
                    } else {
                        let y = list_y_start as u16 - y as u16;
                        let selection = std::cmp::min(SPELLS_TO_SHOW - 1, y / LINE_HEIGHT);
                        self.selection = self.offset + selection as usize;
                    }
                }
                println!("selection {} - offset {}", self.selection, self.offset);
            } else {
                self.mouse_dragging = false;
                self.mouse_dragging_timer = Duration::ZERO;
            }
        }
    }

    fn draw<E: engine::engine::GameEngine>(&mut self, context: &mut crate::game::Context<E>) {
        if !self.visible {
            return;
        }
        self.background.draw(context);

        // draw selection

        context.engine.draw_image(
            DrawImage {
                position: Position::new(
                    self.position.0 + 6,
                    self.position.1 - 5 - LINE_HEIGHT * (self.selection - self.offset + 1) as u16,
                    0.999,
                ),
                color: WHITE,
                index: context.resources.textures.spell_selection,
                ..Default::default()
            },
            Target::UI,
        );

        // draw spells
        let start = std::cmp::min(TOTAL_SPELLS - SPELLS_TO_SHOW as usize, self.offset);
        let end = std::cmp::min(TOTAL_SPELLS, self.offset + SPELLS_TO_SHOW as usize);
        for i in start..end {
            let spell = &mut self.spells[i];
            let selection_index = LINE_HEIGHT * (i as u16 - self.offset as u16 + 1);
            spell.draw(
                context,
                self.position.0 + 8,
                self.position.1 - 2 - selection_index,
            );
        }

        self.cast_button.draw(context);
    }
}
