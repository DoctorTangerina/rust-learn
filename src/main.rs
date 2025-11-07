use eframe::egui;
use egui::{Id, SidePanel, CentralPanel, Ui, panel::Side, Response};

fn add_label(ui: &mut Ui) -> Response {
    ui.add(egui::Label::new("alo"))
}

fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "hello world", 
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    name: String,
    age: u32
}

impl Default for MyApp {
    fn default() -> Self {
        Self { 
            name: "Arthur".to_owned(), 
            age: 22 
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {  
        SidePanel::new(Side::Left, Id::new("sidepanel")).show(ctx, |ui| {
            ui.heading("side bar");
            ui.label("im a sidebar :)");

            CentralPanel::default()
                .show_inside(ui, |ui| {
                    ui.label("AAAAA");
                });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("Hello World");
            ui.horizontal(|ui| {
                let label_name = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(label_name.id);  
            });

            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                self.age += 1;
            }

            if ui.button("Magic!").clicked() {
                add_label(ui);
            }
            
            egui::TopBottomPanel::new(
                egui::panel::TopBottomSide::Bottom,
                Id::new("embaixo"))
                .show_inside(ui, |ui| {
                    ui.label("Bottomfrag");
                });

            ui.label(format!("Hello '{}', age {}", self.name, self.age));

            ui.label(if let Some(cpu) = frame.info().cpu_usage {
                    format!("Cpu: {}", cpu)
                } else {
                    "Cpu: No info".to_string()
                }
            );
        });
    }
}