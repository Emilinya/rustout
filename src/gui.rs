use egui::{Key, Vec2};
use std::{collections::HashSet, time::Instant};

struct Gui {
    key_map: HashSet<Key>,
    player_pos: Vec2,
    last_update: Instant,
    window_size: Vec2,
}

impl Gui {
    fn new(window_size: Vec2) -> Self {
        Self {
            key_map: HashSet::default(),
            player_pos: Vec2::default(),
            last_update: Instant::now(),
            window_size,
        }
    }
}

pub fn run() -> eframe::Result {
    let window_size = Vec2::new(480.0, 360.0);
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size(window_size),
        ..Default::default()
    };
    eframe::run_native(
        "rustout",
        options,
        Box::new(|_cc| Ok(Box::new(Gui::new(window_size)))),
    )
}

impl eframe::App for Gui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let scale = self.window_size.min_elem();

        // read input
        ctx.input(|i| {
            for key in [Key::A, Key::D, Key::W, Key::S] {
                if i.key_down(key) {
                    self.key_map.insert(key);
                } else if i.key_released(key) {
                    self.key_map.remove(&key);
                }
            }
        });

        // update player position
        let dt = self.last_update.elapsed().as_secs_f32();
        self.last_update = Instant::now();
        if self.key_map.contains(&Key::A) {
            self.player_pos.x -= 0.5 * scale * dt;
        }
        if self.key_map.contains(&Key::D) {
            self.player_pos.x += 0.5 * scale * dt;
        }
        if self.key_map.contains(&Key::W) {
            self.player_pos.y -= 0.5 * scale * dt;
        }
        if self.key_map.contains(&Key::S) {
            self.player_pos.y += 0.5 * scale * dt;
        }

        // render player
        egui::CentralPanel::default().show(ctx, |ui| {
            let (response, painter) =
                ui.allocate_painter(ui.available_size(), egui::Sense::hover());

            let rect = response.rect;
            painter.circle_filled(
                rect.center() + self.player_pos,
                scale * 0.1,
                egui::Color32::ORANGE,
            )
        });
        ctx.request_repaint_after_secs(1.0 / 30.0);
    }
}
