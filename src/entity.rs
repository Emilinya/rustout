use rust_training_tool::{
    egui::{Rect, Ui},
    gui::Context,
};

pub trait Entity {
    fn get_bounding_box(&self, ctx: &Context) -> Rect;
    fn draw(&mut self, ctx: &Context, ui: &mut Ui);
    fn reset(&mut self);
}
