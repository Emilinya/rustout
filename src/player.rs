use egui::{Key, Vec2};

use crate::gui::Context;

#[derive(Default)]
pub struct Player {
    pos: Vec2,
}

impl Player {
    pub fn draw(&mut self, ctx: &Context, _ui: &mut egui::Ui) {
        let scale = ctx.drawable_area.size().min_elem();
        let dt = ctx.dt.as_secs_f32();

        if ctx.key_map.contains(&Key::A) {
            self.pos.x -= 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::D) {
            self.pos.x += 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::W) {
            self.pos.y -= 0.5 * scale * dt;
        }
        if ctx.key_map.contains(&Key::S) {
            self.pos.y += 0.5 * scale * dt;
        }

        if let Some(painter) = &ctx.painter {
            painter.circle_filled(
                ctx.drawable_area.center() + self.pos,
                scale * 0.1,
                egui::Color32::ORANGE,
            );
        }
    }
}
