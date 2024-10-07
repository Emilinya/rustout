mod ball;
mod blocks;
mod entity;
mod player;

use crate::entity::Entity;
use rust_training_tool::gui::{run, Context};
use rust_training_tool::{
    eframe, egui,
    egui::{Key, Vec2},
};

fn main() {
    let mut player = player::Player::default();
    let mut blocks = blocks::Blocks::default();
    let mut ball = ball::Ball::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 360.0])
            .with_resizable(false),
        ..Default::default()
    };
    run(options, "rustout", |ctx, ui| {
        if !ball.started && ctx.key_map.contains(&Key::Space) {
            ball.started = true;
            ball.dir = Vec2::new(0.01, -1.0);
            if ctx.key_map.contains(&Key::A) {
                ball.dir.x -= 1.5;
            }
            if ctx.key_map.contains(&Key::D) {
                ball.dir.x += 1.5;
            }
            ball.dir = ball.dir.normalized();
        }
        if ctx.key_map.contains(&Key::R) {
            player.reset();
            blocks.reset();
            ball.reset();
        }

        if blocks
            .blocks
            .as_ref()
            .is_some_and(|b| b.iter().all(|b| !b.alive))
        {
            paint_victory(&ctx);
        } else if ball.dead {
            paint_defeat(&ctx);
        } else {
            player.update(&ctx);
            ball.update(&ctx, &mut blocks, &player);

            player.draw(&ctx, ui);
            blocks.draw(&ctx, ui);
            ball.draw(&ctx, ui);
        }
    })
    .unwrap();
}

fn paint_defeat(ctx: &Context) {
    ctx.painter.rect_filled(
        ctx.drawable_area,
        egui::Rounding::ZERO,
        egui::Color32::DARK_RED,
    );

    let font = egui::FontId {
        size: 40.0,
        ..Default::default()
    };
    ctx.painter.text(
        ctx.drawable_area.center(),
        egui::Align2::CENTER_CENTER,
        "       Game Over\nPress 'R' to restart",
        font,
        egui::Color32::WHITE,
    );
}

fn paint_victory(ctx: &Context) {
    ctx.painter.rect_filled(
        ctx.drawable_area,
        egui::Rounding::ZERO,
        egui::Color32::DARK_GREEN,
    );

    let font = egui::FontId {
        size: 40.0,
        ..Default::default()
    };
    ctx.painter.text(
        ctx.drawable_area.center(),
        egui::Align2::CENTER_CENTER,
        "        You Won!\nPress 'R' to restart",
        font,
        egui::Color32::WHITE,
    );
}
