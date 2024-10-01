use egui::{Key, Rect, Vec2};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use crate::{ball::Ball, blocks::Blocks, entity::Entity, player::Player};

pub struct Context {
    pub dt: Duration,
    pub key_map: HashSet<Key>,
    pub drawable_area: Rect,
    pub painter: Option<egui::Painter>,
}

struct Gui {
    last_update: Instant,
    player: Player,
    blocks: Blocks,
    ball: Ball,
    ctx: Context,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            player: Player::default(),
            blocks: Blocks::default(),
            ball: Ball::default(),
            ctx: Context {
                dt: Duration::ZERO,
                key_map: HashSet::new(),
                drawable_area: Rect::ZERO,
                painter: None,
            },
        }
    }
}

pub fn run() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 360.0])
            .with_resizable(false),
        ..Default::default()
    };
    eframe::run_native(
        "rustout",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(Gui::default()))
        }),
    )
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.input(|i| {
            for key in [Key::A, Key::D, Key::W, Key::S] {
                if i.key_pressed(key) {
                    self.ctx.key_map.insert(key);
                } else if i.key_released(key) {
                    self.ctx.key_map.remove(&key);
                }
            }
            if !self.ball.started && i.key_pressed(Key::Space) {
                self.ball.started = true;
                self.ball.dir = Vec2::new(0.01, -1.0);
                if self.ctx.key_map.contains(&Key::A) {
                    self.ball.dir.x -= 1.5;
                }
                if self.ctx.key_map.contains(&Key::D) {
                    self.ball.dir.x += 1.5;
                }
                self.ball.dir = self.ball.dir.normalized();
            }
            if i.key_pressed(Key::R) {
                self.player.reset();
                self.blocks.reset();
                self.ball.reset();
            }
        });

        self.ctx.dt = self.last_update.elapsed();
        self.last_update = Instant::now();

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                self.ctx.drawable_area = response.rect;
                self.ctx.painter = Some(painter);

                if self
                    .blocks
                    .blocks
                    .as_ref()
                    .is_some_and(|b| b.iter().all(|b| !b.alive))
                {
                    self.paint_victory();
                } else if self.ball.dead {
                    self.paint_defeat();
                } else {
                    self.player.update(&self.ctx);
                    self.ball.update(&self.ctx, &mut self.blocks, &self.player);

                    self.player.draw(&self.ctx, ui);
                    self.blocks.draw(&self.ctx, ui);
                    self.ball.draw(&self.ctx, ui);
                }
            });

        // update at 30 fps
        ctx.request_repaint_after_secs(1.0 / 30.0);
    }
}

impl Gui {
    fn paint_defeat(&self) {
        let painter = self.ctx.painter.as_ref().unwrap();
        painter.rect_filled(
            self.ctx.drawable_area,
            egui::Rounding::ZERO,
            egui::Color32::DARK_RED,
        );

        let font = egui::FontId {
            size: 40.0,
            ..Default::default()
        };
        painter.text(
            self.ctx.drawable_area.center(),
            egui::Align2::CENTER_CENTER,
            "       Game Over\nPress 'R' to restart",
            font,
            egui::Color32::WHITE,
        );
    }

    fn paint_victory(&self) {
        let painter = self.ctx.painter.as_ref().unwrap();
        painter.rect_filled(
            self.ctx.drawable_area,
            egui::Rounding::ZERO,
            egui::Color32::DARK_GREEN,
        );

        let font = egui::FontId {
            size: 40.0,
            ..Default::default()
        };
        painter.text(
            self.ctx.drawable_area.center(),
            egui::Align2::CENTER_CENTER,
            "        You Won!\nPress 'R' to restart",
            font,
            egui::Color32::WHITE,
        );
    }
}
