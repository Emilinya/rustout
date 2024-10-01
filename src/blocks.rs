use egui::{Rect, Vec2};

use crate::{entity::Entity, gui::Context};

pub struct Blocks {
    pub blocks: Option<Vec<Block>>,
    block_size: Vec2,
}

#[derive(Debug)]
pub struct Block {
    pub pos: Vec2,
    pub size: Vec2,
    pub alive: bool,
    pub color: egui::Color32,
}

impl Entity for Block {
    fn get_bounding_box(&self, _ctx: &Context) -> Rect {
        Rect::from_center_size(self.pos.to_pos2(), self.size)
    }

    fn draw(&mut self, ctx: &Context, _ui: &mut egui::Ui) {
        if !self.alive {
            return;
        }

        if let Some(painter) = ctx.painter.as_ref() {
            painter.rect_filled(self.get_bounding_box(ctx), egui::Rounding::ZERO, self.color);
        }
    }

    fn reset(&mut self) {
        self.alive = true;
    }
}

impl Default for Blocks {
    fn default() -> Self {
        let block_count = 9.0;
        let ratio = 4.0;
        let width = 1.0 / (block_count + (block_count - 1.0) / 10.0);

        Self {
            blocks: None,
            block_size: Vec2::new(width, width / ratio),
        }
    }
}

impl Entity for Blocks {
    fn get_bounding_box(&self, _ctx: &Context) -> Rect {
        Rect::ZERO
    }

    fn draw(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        if self.blocks.is_none() {
            self.blocks = Some(self.generate_blocks(ctx));
        }

        if let Some(blocks) = self.blocks.as_mut() {
            for block in blocks.iter_mut() {
                block.draw(ctx, ui);
            }
        }
    }

    fn reset(&mut self) {
        if let Some(blocks) = &mut self.blocks {
            for block in blocks.iter_mut() {
                block.reset();
            }
        }
    }
}

impl Blocks {
    fn generate_blocks(&self, ctx: &Context) -> Vec<Block> {
        let padding = self.block_size.x / 10.0;
        let scale = ctx.drawable_area.width();

        let rows = 8;
        let blocks_per_row = ((10.0 + self.block_size.x) / (11.0 * self.block_size.x)) as usize;
        let mut blocks = Vec::with_capacity(rows * blocks_per_row);

        let mut i = 0;
        let block_size = self.block_size * scale;

        let mut y = ctx.drawable_area.top() + 0.5 * block_size.y;
        for _ in 0..rows {
            let mut x = ctx.drawable_area.left() + 0.5 * block_size.x;
            for _ in 0..blocks_per_row {
                let f = i as f32 / (blocks.capacity() - 1) as f32;
                let byte = (f * 255.0) as u8;

                blocks.push(Block {
                    pos: Vec2::new(x, y),
                    size: block_size,
                    alive: true,
                    color: egui::Color32::from_rgb(0, byte, 255 - byte),
                });
                x += (self.block_size.x + padding) * scale;
                i += 1;
            }
            y += (self.block_size.y + padding) * scale;
        }
        blocks
    }
}
