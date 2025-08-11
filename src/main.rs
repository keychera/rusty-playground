#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

// vibed by chatgpt
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use std::time::{Duration, Instant};

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        // Limit FPS by default here if desired:
        vsync: true,
        ..Default::default()
    };
    eframe::run_native(
        "FPS Example",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    last_frame: Instant,
    fps_history: Vec<f64>,
    fps_limit: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            last_frame: Instant::now(),
            fps_history: Vec::with_capacity(200), // keep last 200 FPS samples
            fps_limit: false,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // --- FPS CALC ---
        let now = Instant::now();
        let dt = now.duration_since(self.last_frame);
        self.last_frame = now;
        let fps = if dt.as_secs_f64() > 0.0 {
            1.0 / dt.as_secs_f64()
        } else {
            0.0
        };

        // store in history
        if self.fps_history.len() >= 200 {
            self.fps_history.remove(0); // drop oldest
        }
        self.fps_history.push(fps);

        // --- UI ---
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("FPS Monitor");

            ui.label(format!("Current FPS: {:.1}", fps));

            // Toggle button
            ui.checkbox(&mut self.fps_limit, "Limit FPS?");

            // Plot
            let points: PlotPoints = self
                .fps_history
                .iter()
                .enumerate()
                .map(|(i, &y)| [i as f64, y])
                .collect();
            let line = Line::new("fps_graph",points);
            Plot::new("fps_plot")
                .height(150.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(line);
                });
        });

        // --- FPS LIMIT ---
        if self.fps_limit {
            std::thread::sleep(Duration::from_millis(16)); // ~60 FPS cap
        }

        ctx.request_repaint(); // continuously repaint
    }
}
