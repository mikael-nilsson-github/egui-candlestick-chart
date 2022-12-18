use egui::{
    Stroke,
    Color32,
    plot::{
        Plot,
        BoxPlot,
        BoxElem,
        BoxSpread,
    }
};

pub struct App {
    value: f64,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            value: 42.0,
        }
    }
}

fn candlestick_chart(ui: &mut egui::Ui) {
    let red = Color32::from_rgb(255,0,0);
    let green = Color32::from_rgb(0,255,0);
    let data = BoxPlot::new(vec![
        BoxElem::new(1.0, BoxSpread::new(1.5, 2.2, 2.2, 2.6, 3.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(1.5, BoxSpread::new(1.5, 2.4, 2.4, 2.8, 3.5)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(2.0, BoxSpread::new(1.8, 2.0, 2.4, 2.5, 2.7)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
        BoxElem::new(2.5, BoxSpread::new(1.5, 1.8, 1.8, 2.1, 2.2)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.0, BoxSpread::new(1.4, 1.6, 1.6, 1.8, 2.1)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
        BoxElem::new(3.5, BoxSpread::new(0.5, 1.5, 1.5, 1.6, 1.7)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
        BoxElem::new(4.0, BoxSpread::new(1.2, 1.4, 1.4, 2.9, 3.2)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(4.5, BoxSpread::new(2.1, 2.3, 2.3, 2.6, 2.7)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(5.0, BoxSpread::new(1.9, 2.1, 2.1, 2.7, 3.5)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
        BoxElem::new(5.5, BoxSpread::new(2.0, 2.1, 2.1, 2.9, 3.3)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.0, BoxSpread::new(2.3, 2.9, 2.9, 3.7, 4.1)).whisker_width(0.0).fill(green).stroke(Stroke::new(2.0, green)),
        BoxElem::new(6.5, BoxSpread::new(3.1, 3.4, 3.4, 4.0, 4.2)).whisker_width(0.0).fill(red).stroke(Stroke::new(2.0, red)),
    ]);

    let plot = Plot::new("candlestick chart")
        .view_aspect(2.0);

    plot.show(ui, |plot_ui| {
            plot_ui.box_plot(data);
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("value = {}", self.value));
        });
        egui::Window::new("My Window").show(ctx, |ui| {
            candlestick_chart(ui);
        }); 
    }
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "candlestic chart",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}
