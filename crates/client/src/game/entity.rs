use ::rand::{seq::IteratorRandom, thread_rng};
use ao::ao_20::init::Body;
use macroquad::prelude::*;

use crate::{app::resources::Resources, error::RuntimeError};

use super::{
    camera::{create_entity_camera, ENTITY_SIZE},
    Game,
};

pub struct Entity {
    body: usize,
    head: usize,
    // shield: usize,
    // gear: usize,
    weapon: usize,
    name: String,
    pub position: Vec2,
    camera: Camera2D,
    render_state: RenderState,
}

const TRANSPARENCY: f32 = 1.;

pub enum RenderState {
    Render,
    Ready,
}

impl Default for Entity {
    fn default() -> Self {
        Self {
            body: 1,
            head: 1,
            // shield: 0,
            // gear: 0,
            weapon: 1,
            position: vec2(50., 50.),
            name: "Pandora".to_string(),
            camera: create_entity_camera(),
            render_state: RenderState::Render,
        }
    }
}

impl Entity {
    pub fn random(resources: &Resources) -> Self {
        let mut rng = thread_rng();
        let random_body = loop {
            let (id, body) = resources.bodies.iter().choose(&mut rng).unwrap();

            match body {
                Body::AnimatedWithTemplate {
                    template_id,
                    file_num,
                    head_offset,
                } => {
                    if let Some(template) = resources.body_templates.get(template_id) {
                        if template.width == 0 {
                            continue;
                        }
                        if file_num != &0 && head_offset.1 != 0 {
                            break *id;
                        }
                    }
                }
                Body::Animated { walks, head_offset } => {
                    let first_animation = walks.0;
                    if head_offset.1 == 0 {
                        continue;
                    }
                    if let Ok(animation) = resources.get_animation(&first_animation.to_string()) {
                        let first_image = &animation.frames[0];
                        if resources.images.contains_key(first_image) {
                            break *id;
                        }
                    }
                }
            }
        };
        let random_head = loop {
            let (id, head) = resources.heads.iter().choose(&mut rng).unwrap();
            if head.0 != 0 {
                break *id;
            }
        };
        let random_weapon = resources.weapons.iter().choose(&mut rng).unwrap();
        let x = rand::gen_range(0, 100);
        let y = rand::gen_range(0, 100);
        Self {
            body: random_body,
            head: random_head,
            weapon: *random_weapon.0,
            position: vec2(x as f32, y as f32),
            ..Default::default()
        }
    }

    pub fn get_texture(&self) -> Texture2D {
        self.camera.render_target.unwrap().texture
    }
}

impl Game {
    async fn draw_cached_entity(&mut self, id: usize) {
        let entity = self.entities.get(&id).unwrap();

        if matches!(entity.render_state, RenderState::Render) || self.screen_size_dirty {
            set_camera(&entity.camera);
            clear_background(BLANK);

            if entity.body != 0 {
                let x = (ENTITY_SIZE.0 / 2) as f32;
                let y = (ENTITY_SIZE.1 - 20) as f32;

                let head_offset = match self.resources.bodies.get(&entity.body) {
                    Some(Body::Animated { walks, head_offset }) => {
                        let body_grh = walks.0;
                        _ = self
                            .draw_animated_grh(body_grh, x, y, TRANSPARENCY, None)
                            .await;

                        head_offset
                    }
                    Some(Body::AnimatedWithTemplate {
                        template_id,
                        file_num,
                        head_offset,
                    }) => {
                        if let Some(template) = self.resources.body_templates.get(template_id) {
                            _ = self
                                .draw_animated_template(template_id, file_num, template, x, y, 1.)
                                .await;
                        }
                        head_offset
                    }
                    None => &(0, 0),
                };
                if entity.head != 0 {
                    if let Some(head) = self.resources.heads.get(&entity.head) {
                        let x = (x as isize + head_offset.0) as f32;
                        let y = (y as isize + head_offset.1) as f32;
                        _ = self.draw_grh(&head.2.to_string(), x, y, 1., None).await;
                    }
                }
            }
        }

        set_camera(&self.world_camera);
        let x = entity.position.x * 32.;
        let y = entity.position.y * 32.;
        let font = self.resources.fonts.tahoma;
        if self.settings.draw_names {
            draw_name(&entity.name, font, RED, x, y);
        }

        let mut color = WHITE;
        color.a = TRANSPARENCY;
        draw_texture_ex(
            entity.get_texture(),
            x - ENTITY_SIZE.0 as f32 / 2.,
            y - ENTITY_SIZE.1 as f32,
            color,
            DrawTextureParams {
                flip_y: true,
                ..Default::default()
            },
        );

        self.entities.get_mut(&id).unwrap().render_state = RenderState::Ready;
    }

    async fn draw_noncached_entity(&mut self, id: usize) {
        let entity = self.entities.get(&id).unwrap();
        let Vec2 { x, y } = entity.position;
        let x = x * 32.;
        let y = y * 32.;

        if entity.body != 0 {
            let y = y - 20.;
            let head_offset = match self.resources.bodies.get(&entity.body) {
                Some(Body::Animated { walks, head_offset }) => {
                    let body_grh = walks.0;
                    _ = self
                        .draw_animated_grh(body_grh, x, y, TRANSPARENCY, None)
                        .await;

                    head_offset
                }
                Some(Body::AnimatedWithTemplate {
                    template_id,
                    file_num,
                    head_offset,
                }) => {
                    if let Some(template) = self.resources.body_templates.get(template_id) {
                        _ = self
                            .draw_animated_template(
                                template_id,
                                file_num,
                                template,
                                x,
                                y,
                                TRANSPARENCY,
                            )
                            .await;
                    }
                    head_offset
                }
                None => &(0, 0),
            };
            if entity.head != 0 {
                if let Some(head) = self.resources.heads.get(&entity.head) {
                    let x = x + head_offset.0 as f32;
                    let y = y + head_offset.1 as f32;
                    _ = self
                        .draw_grh(&head.2.to_string(), x, y, TRANSPARENCY, None)
                        .await;
                }
            }
        }

        let font = self.resources.fonts.tahoma;

        if self.settings.draw_names {
            draw_name(&entity.name, font, RED, x, y);
        }
    }

    pub async fn draw_entity(&mut self, id: usize) {
        if self.settings.cache_entities {
            self.draw_cached_entity(id).await;
        } else {
            self.draw_noncached_entity(id).await;
        }
    }

    async fn draw_animated_template(
        &self,
        template_id: &usize,
        file_num: &usize,
        template: &ao::ao_20::init::parse::template::Template,
        x: f32,
        y: f32,
        transparency: f32,
    ) -> Result<(), RuntimeError> {
        let id = format!("{template_id}-{file_num}-0");
        let image = ao::ao_20::graphics::Image {
            file_num: *file_num as u32,
            x: template.x as u16,
            y: template.y as u16,
            width: template.width as u16,
            height: template.height as u16,
            id: id.to_string(),
        };
        self.draw_image(&image, x, y, transparency, None).await
    }
}

fn draw_name(name: &str, font: Font, color: Color, x: f32, y: f32) {
    let size = 12;
    let params = TextParams {
        font,
        font_size: size,
        color,
        ..Default::default()
    };
    let d = measure_text(name, Some(font), size, 1.);
    draw_text_ex(name, x - d.width / 2., y - d.height, params);
}
