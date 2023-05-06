use std::time::Duration;

use eframe::egui::{
    self,
    plot::{Legend, Plot, PlotPoint, PlotUi, Points},
    InnerResponse,
};
use rrt::RRT;
mod rrt;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(800.0, 600.0)),
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default)]
enum AppState {
    #[default]
    Emtyp,
    OneClicked(PlotPoint),
    Started(RRT),
}

#[derive(Default)]
struct MyApp {
    state: AppState,
}

impl AppState {
    fn handle_click(&mut self, point: &PlotPoint) {
        match self {
            AppState::Emtyp => *self = Self::OneClicked(*point),
            AppState::OneClicked(start) => {
                let mut rrt = RRT::new(start, point);
                rrt.start();
                *self = Self::Started(rrt);
            }
            _ => (),
        }
    }

    fn update(&mut self, ctx: &egui::Context) {
        if let AppState::Started(_) = self {
            if ctx.input(|e| e.key_pressed(egui::Key::R)) {
                *self = AppState::Emtyp
            }
        }
    }

    fn draw(&self, plot_ui: &mut PlotUi) {
        match self {
            AppState::Emtyp => (),
            AppState::OneClicked(point) => plot_ui.points(Points::new(vec![[point.x, point.y]])),
            AppState::Started(rrt) => {
                rrt.draw(plot_ui);
            }
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.state.update(ctx);
            let plot = Plot::new("RRT algoritmasi gorsellestirme")
                .height(600.)
                .legend(Legend::default())
                .view_aspect(1.)
                .data_aspect(1.);
            let InnerResponse {
                inner: (point, clicked),
                response: _,
            } = plot.show(ui, |plot_ui| {
                self.state.draw(plot_ui);
                (plot_ui.pointer_coordinate(), plot_ui.plot_clicked())
            });

            if clicked && point.is_some() {
                self.state.handle_click(&point.unwrap());
                println!("clicked to plot");
            }
        });

        ctx.request_repaint_after(Duration::from_secs((1. / 60.) as u64));
    }
}
