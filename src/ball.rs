use std::collections::HashSet;

use crate::{blocks::Blocks, entity::Entity, player::Player};

use egui::{Rect, Vec2};
use rust_training_tool::{
    collision::{collide_with_boundary, collide_with_rect, BounceDirection},
    egui,
    gui::Context,
};

const SIZE: f32 = 0.05;

#[derive(Default)]
pub struct Ball {
    pos: Vec2,
    pub dir: Vec2,
    pub dead: bool,
    pub started: bool,
}

impl Entity for Ball {
    fn get_bounding_box(&self, ctx: &Context) -> Rect {
        let size = SIZE * ctx.drawable_area.width();
        Rect::from_center_size(ctx.drawable_area.center() + self.pos, Vec2::new(size, size))
    }

    fn draw(&mut self, ctx: &Context, ui: &mut egui::Ui) {
        egui::Image::new(egui::include_image!("../rust.png"))
            .paint_at(ui, self.get_bounding_box(ctx));
    }

    fn reset(&mut self) {
        self.pos = Vec2::ZERO;
        self.dir = Vec2::ZERO;
        self.dead = false;
        self.started = false;
    }
}

impl Ball {
    pub fn update(&mut self, ctx: &Context, blocks: &mut Blocks, player: &Player) {
        let scale = ctx.drawable_area.width();

        if !self.started {
            self.pos = Vec2::new(
                player.pos.x,
                0.5 * ctx.drawable_area.bottom() - 2.0 * SIZE * scale,
            );
            return;
        }

        let speed = 1.0 * scale;

        let dt = ctx.dt.as_secs_f32();
        self.pos += self.dir * speed * dt;

        self.boundary_bounce(ctx);
        self.block_bounce(ctx, blocks);
        self.player_bounce(ctx, player);
    }

    fn boundary_bounce(&mut self, ctx: &Context) {
        let shape = self.get_bounding_box(ctx);
        let area = ctx.drawable_area;

        if let Some(dir) = collide_with_boundary(&shape, &area) {
            let mut center_shape = shape;
            center_shape.set_center(ctx.drawable_area.center());

            match dir {
                BounceDirection::Right => {
                    self.pos.x = area.left() - center_shape.left();
                    self.dir.x = -self.dir.x;
                }
                BounceDirection::Left => {
                    self.pos.x = area.right() - center_shape.right();
                    self.dir.x = -self.dir.x;
                }
                BounceDirection::Down => {
                    self.pos.y = area.top() - center_shape.top();
                    self.dir.y = -self.dir.y;
                }
                BounceDirection::Up => {
                    self.dead = true;
                }
            }
        }
    }

    fn block_bounce(&mut self, ctx: &Context, blocks: &mut Blocks) {
        let mut collision_directions = HashSet::new();
        if let Some(blocks) = &mut blocks.blocks {
            for block in blocks.iter_mut() {
                if !block.alive {
                    continue;
                }
                if let Some(dir) = collide_with_rect(
                    &self.dir,
                    &self.get_bounding_box(ctx),
                    &block.get_bounding_box(ctx),
                ) {
                    block.alive = false;
                    collision_directions.insert(dir);
                }
            }
        }

        for dir in collision_directions {
            match dir {
                BounceDirection::Up | BounceDirection::Down => self.dir.y = -self.dir.y,
                BounceDirection::Left | BounceDirection::Right => self.dir.x = -self.dir.x,
            }
        }
    }

    fn player_bounce(&mut self, ctx: &Context, player: &Player) {
        let self_rect = self.get_bounding_box(ctx);
        let player_rect = player.get_bounding_box(ctx);
        if self_rect.intersects(player_rect) {
            let between = (self_rect.center() - player_rect.center()).normalized();
            self.dir.y = between.y;
            self.dir.x += between.x;
            self.dir = self.dir.normalized();
        }
    }
}
