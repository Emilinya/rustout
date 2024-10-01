use egui::{Rect, Vec2};

use std::collections::HashSet;

use crate::{
    blocks::{Block, Blocks},
    entity::Entity,
    gui::Context,
    player::Player,
};

const SIZE: f32 = 0.05;

#[derive(Default)]
pub struct Ball {
    pos: Vec2,
    pub dir: Vec2,
    pub dead: bool,
    pub started: bool,
}

#[derive(Debug, Eq, PartialEq, Hash)]
enum BounceDir {
    Top,
    Left,
    Bottom,
    Right,
}

impl Entity for Ball {
    fn get_bounding_box(&self, ctx: &Context) -> Rect {
        let size = SIZE * ctx.drawable_area.width();
        Rect::from_center_size(ctx.drawable_area.center() + self.pos, Vec2::new(size, size))
    }

    fn draw(&mut self, ctx: &Context) {
        if let Some(painter) = &ctx.painter {
            painter.rect_filled(
                self.get_bounding_box(ctx),
                egui::Rounding::ZERO,
                egui::Color32::GRAY,
            );
        }
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

        let mut center_shape = shape;
        center_shape.set_center(ctx.drawable_area.center());

        if !area.contains_rect(shape) {
            if shape.left() < area.left() {
                self.pos.x = area.left() - center_shape.left();
                self.dir.x = -self.dir.x;
            } else if shape.right() > area.right() {
                self.pos.x = area.right() - center_shape.right();
                self.dir.x = -self.dir.x;
            }
            if shape.top() < area.top() {
                self.pos.y = area.top() - center_shape.top();
                self.dir.y = -self.dir.y;
            } else if shape.bottom() > area.bottom() {
                self.dead = true;
            }
        }
    }

    fn block_bounce(&mut self, ctx: &Context, blocks: &mut Blocks) {
        let mut collision_directions = HashSet::new();
        if let Some(blocks) = &mut blocks.blocks {
            for block in blocks.iter_mut() {
                if let Some(dir) = self.collide_with_block(ctx, block) {
                    collision_directions.insert(dir);
                }
            }
        }

        for dir in collision_directions {
            match dir {
                BounceDir::Top | BounceDir::Bottom => self.dir.y = -self.dir.y,
                BounceDir::Left | BounceDir::Right => self.dir.x = -self.dir.x,
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

    fn collide_with_block(&mut self, ctx: &Context, block: &mut Block) -> Option<BounceDir> {
        if !block.alive {
            return None;
        }
        let self_rect = self.get_bounding_box(ctx);
        let block_rect = Rect::from_center_size(block.pos.to_pos2(), block.size);

        let mut center_shape = self_rect;
        center_shape.set_center(ctx.drawable_area.center());

        if self_rect.intersects(block_rect) {
            block.alive = false;
            let inside_bottom = -(self_rect.top() - block_rect.bottom());
            let inside_right = -(self_rect.left() - block_rect.right());
            let inside_left = self_rect.right() - block_rect.left();
            let inside_top = self_rect.bottom() - block_rect.top();

            let mut min_in: f32 = 999.0;
            if inside_bottom >= 0.0 && self.dir.y < 0.0 {
                min_in = min_in.min(inside_bottom);
            }
            if inside_top >= 0.0 && self.dir.y > 0.0 {
                min_in = min_in.min(inside_top);
            }
            if inside_left >= 0.0 && self.dir.x > 0.0 {
                min_in = min_in.min(inside_left);
            }
            if inside_right >= 0.0 && self.dir.x < 0.0 {
                min_in = min_in.min(inside_right);
            }

            if min_in == inside_bottom {
                Some(BounceDir::Bottom)
            } else if min_in == inside_top {
                Some(BounceDir::Top)
            } else if min_in == inside_left {
                Some(BounceDir::Left)
            } else if min_in == inside_right {
                Some(BounceDir::Right)
            } else {
                eprintln!("WARNING: colliding but not inside?");
                block.alive = true;
                None
            }
        } else {
            None
        }
    }
}
