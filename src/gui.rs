use egui::{Key, Rect};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use crate::player::Player;

pub struct Context {
    pub dt: Duration,
    pub key_map: HashSet<Key>,
    pub drawable_area: Rect,
    pub painter: Option<egui::Painter>,
}

struct Gui {
    last_update: Instant,
    player: Player,
    ctx: Context,
}

impl Default for Gui {
    fn default() -> Self {
        Self {
            last_update: Instant::now(),
            player: Player::default(),
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
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rustout",
        options,
        Box::new(|_cc| Ok(Box::new(Gui::default()))),
    )
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // read input
        ctx.input(|i| {
            for key in [Key::A, Key::D, Key::W, Key::S] {
                if i.key_down(key) {
                    self.ctx.key_map.insert(key);
                } else if i.key_released(key) {
                    self.ctx.key_map.remove(&key);
                }
            }
        });

        // update dt
        self.ctx.dt = self.last_update.elapsed();
        self.last_update = Instant::now();

        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                // create painter
                let (response, painter) =
                    ui.allocate_painter(ui.available_size(), egui::Sense::hover());
                self.ctx.drawable_area = response.rect;
                self.ctx.painter = Some(painter);

                // render player
                self.player.draw(&self.ctx, ui);
            });

        // update at 30 fps
        ctx.request_repaint_after_secs(1.0 / 30.0);
    }
}
