use egui::{Rect, Vec2};

use crate::gui::Context;

pub struct Blocks {
    pub blocks: Option<Vec<Block>>,
    block_size: Vec2,
}

#[derive(Debug)]
pub struct Block {
    pub pos: Vec2,
    pub size: Vec2,
    pub alive: bool,
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

impl Blocks {
    pub fn draw(&mut self, ctx: &Context) {
        let area = ctx.drawable_area;
        let scale = area.width();

        if self.blocks.is_none() {
            self.blocks = Some(self.generate_blocks(ctx));
        }

        let Some(painter) = &ctx.painter else { return };
        let blocks = self.blocks.as_ref().unwrap();
        for (i, block) in blocks.iter().enumerate() {
            if !block.alive {
                continue;
            }

            let f = i as f32 / (blocks.len() - 1) as f32;
            let byte = (f * 255.0) as u8;

            let shape = Rect::from_center_size(block.pos.to_pos2(), self.block_size * scale);
            painter.rect_filled(
                shape,
                egui::Rounding::ZERO,
                egui::Color32::from_rgb(0, byte, 255 - byte),
            );
        }
    }

    fn generate_blocks(&self, ctx: &Context) -> Vec<Block> {
        let padding = self.block_size.x / 10.0;
        let scale = ctx.drawable_area.width();

        let rows = 8;
        let blocks_per_row = ((10.0 + self.block_size.x) / (11.0 * self.block_size.x)) as usize;
        let mut blocks = Vec::with_capacity(rows * blocks_per_row);

        let block_size = self.block_size * scale;

        let mut y = ctx.drawable_area.top() + 0.5 * block_size.y;
        for _ in 0..rows {
            let mut x = ctx.drawable_area.left() + 0.5 * block_size.x;
            for _ in 0..blocks_per_row {
                blocks.push(Block {
                    pos: Vec2::new(x, y),
                    size: block_size,
                    alive: true,
                });
                x += (self.block_size.x + padding) * scale;
            }
            y += (self.block_size.y + padding) * scale;
        }
        blocks
    }

    pub fn reset(&mut self) {
        if let Some(blocks) = &mut self.blocks {
            for block in blocks.iter_mut() {
                block.alive = true;
            }
        }
    }
}
