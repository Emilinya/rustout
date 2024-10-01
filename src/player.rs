use egui::{Key, Rect, Vec2};

use crate::{entity::Entity, gui::Context};

const RATIO: f32 = 8.0;
const HEIGHT: f32 = 0.03;

#[derive(Default)]
pub struct Player {
    pub pos: Vec2,
}

impl Entity for Player {
    fn get_bounding_box(&self, ctx: &Context) -> Rect {
        let height = HEIGHT * ctx.drawable_area.width();
        let width = height * RATIO;
        Rect::from_center_size(
            ctx.drawable_area.center_bottom() - Vec2::new(0.0, height) + self.pos,
            Vec2::new(width, height),
        )
    }

    fn draw(&mut self, ctx: &Context, _ui: &mut egui::Ui) {
        if let Some(painter) = &ctx.painter {
            painter.rect_filled(
                self.get_bounding_box(ctx),
                egui::Rounding::ZERO,
                egui::Color32::ORANGE,
            );
        }
    }

    fn reset(&mut self) {
        self.pos = Vec2::ZERO;
    }
}

impl Player {
    pub fn update(&mut self, ctx: &Context) {
        let area = ctx.drawable_area;
        let scale = area.width();
        let dt = ctx.dt.as_secs_f32();

        if ctx.key_map.contains(&Key::A) {
            self.pos.x -= scale * dt;
        }
        if ctx.key_map.contains(&Key::D) {
            self.pos.x += scale * dt;
        }

        let mut bounding_box = self.get_bounding_box(ctx);
        bounding_box.set_center(area.center());

        let min = area.left() - bounding_box.left();
        let max = area.right() - bounding_box.right();
        self.pos.x = self.pos.x.clamp(min, max);
    }
}
