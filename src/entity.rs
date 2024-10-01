use crate::gui::Context;
use egui::Rect;

pub trait Entity {
    fn get_bounding_box(&self, ctx: &Context) -> Rect;
    fn draw(&mut self, ctx: &Context);
    fn reset(&mut self);
}
