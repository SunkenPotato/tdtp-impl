mod intervallvis; // Importiert die Datei intervallvis.rs

use eframe::{egui, epi};
use intervallvis::LivePlot;

struct MyApp {
    plot: LivePlot,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            plot: LivePlot::new(2), // alle 2 Sekunden neuer Wert
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str { "Live Plot mit Intervallen" }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        // Werte updaten
        let _ = self.plot.update();

        // GUI zeichnen
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Live Plot – letzte Messung: {:.2}", self.plot.data.last().unwrap_or(&0.0)));
            
            let plot = egui::plot::Plot::new("live_plot").height(300.0);
            plot.show(ui, |plot_ui| {
                let points: Vec<_> = self.plot.data.iter().enumerate()
                    .map(|(i, &v)| [i as f64, v]).collect();
                plot_ui.line(egui::plot::Line::new(egui::plot::PlotPoints::from(points)));
            });
        });

        // Nächstes Frame erzwingen
        ctx.request_repaint();
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Live Plot mit Intervallen",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}
