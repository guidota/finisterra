use roma::{
    get_delta,
    ui::{self, ManagedTextureId},
};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct UI {
    pub main_texture: ManagedTextureId,
    pub top_panel_size: usize,
    pub right_panel_size: usize,
    pub border: usize,
}

impl UI {
    pub fn resize(&mut self, (width, height): (usize, usize)) {
        self.border = match (width, height) {
            (WINDOW_WIDTH, WINDOW_HEIGHT) => 10,
            // (1920, 1080) => 15,
            _ => 15,
        };
        self.right_panel_size = (width as f32 / 2.5) as usize - self.border * 2;
        self.top_panel_size = height / 6;
    }

    pub fn draw(&mut self, _window_size: (usize, usize), _render_size: (usize, usize)) {
        let delta = get_delta();
        ui::align(ui::Alignment::TOP_RIGHT, || {
            let mut textbox =
                ui::widgets::TextBox::new(format!("FPS: {:.2}", 1. / delta.as_secs_f32()));
            textbox.fill = None;
            textbox.style.font_size = 10.;
            textbox.style.font = ui::font::FontName::new("tahoma");
            textbox.style.color = ui::Color::YELLOW;
            textbox.style.align = ui::style::TextAlignment::End;

            textbox.show();
        });
        // ui::image(
        //     self.main_texture,
        //     Vec2::new(window_size.0 as f32, window_size.1 as f32),
        // );
        // ui::row(|| {
        //     let mut col = ui::widgets::List::column();
        //     col.main_axis_alignment = ui::MainAxisAlignment::Start;
        //     col.cross_axis_alignment = ui::CrossAxisAlignment::Stretch;
        //
        //     col.show(|| {
        //         ui::row(|| {
        //             ui::colored_box_container(ui::Color::CLEAR, || {
        //                 ui::scroll_vertical(|| {
        //                     ui::pad(ui::widgets::Pad::all(self.border as f32), || {
        //                         let width = render_size.0;
        //                         ui::colored_box(
        //                             ui::Color::CLEAR,
        //                             Vec2::new(
        //                                 width as f32,
        //                                 (self.top_panel_size - self.border) as f32,
        //                             ),
        //                         );
        //                     });
        //                 });
        //             });
        //             ui::align(ui::Alignment::TOP_RIGHT, || {
        //                 let mut textbox = ui::widgets::TextBox::new(format!(
        //                     "FPS: {:.2}",
        //                     1. / delta.as_secs_f32()
        //                 ));
        //                 textbox.fill = None;
        //                 textbox.style.font_size = 10.;
        //                 textbox.style.font = ui::font::FontName::new("tahoma");
        //                 textbox.style.color = ui::Color::YELLOW;
        //                 textbox.style.align = ui::style::TextAlignment::End;
        //
        //                 textbox.show();
        //             });
        //             ui::align(ui::Alignment::TOP_LEFT, || {
        //                 let mut textbox = ui::widgets::TextBox::new("Finisterra v0.1");
        //                 textbox.fill = None;
        //                 textbox.style.font_size = 10.;
        //                 textbox.style.font = ui::font::FontName::new("tahoma");
        //                 textbox.style.color = ui::Color::CYAN;
        //                 textbox.style.align = ui::style::TextAlignment::Start;
        //
        //                 textbox.show();
        //             });
        //         });
        //         // ui::row(|| {
        //         // ui::expanded(|| {
        //
        //         // });
        //         // });
        //     });
        //
        //     let mut col = ui::widgets::List::column();
        //     col.main_axis_alignment = ui::MainAxisAlignment::Center;
        //     col.cross_axis_alignment = ui::CrossAxisAlignment::Stretch;
        //
        //     ui::constrained(
        //         ui::Constraints::tight(Vec2::new(
        //             (self.right_panel_size + self.border) as f32,
        //             (window_size.1 + self.border * 2) as f32,
        //         )),
        //         || {
        //             ui::expanded(|| {
        //                 col.show(|| {
        //                     ui::center(|| {
        //                         let mut button = ui::widgets::Button::styled("Lanzar");
        //                         button.border_radius = 0.;
        //                         button.padding = ui::widgets::Pad::balanced(60., 10.);
        //                         button.style.text.font = ui::font::FontName::new("tahoma");
        //                         button.hover_style.text.font = ui::font::FontName::new("tahoma");
        //                         button.down_style.text.font = ui::font::FontName::new("tahoma");
        //
        //                         button.show();
        //                     });
        //                 });
        //             });
        //         },
        //     );
        // });
    }
}
