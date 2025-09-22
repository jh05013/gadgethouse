use crate::house::*;
use egui::Id;

#[derive(Default)]
pub struct MyApp {
    next_id: usize,
    pub instances: Vec<(Id, bool, Box<dyn HouseGadget>)>,
}

impl MyApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        cc.egui_ctx.set_visuals(egui::Visuals::light());

        // image loader
        egui_extras::install_image_loaders(&cc.egui_ctx);

        Default::default()
    }

    pub fn new_id(&mut self) -> Id {
        self.next_id += 1;
        Id::new(self.next_id)
    }
}

impl eframe::App for MyApp {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        /*
        egui::TopBottomPanel::top("my_panel").show(ctx, |ui| {
            egui::widgets::global_theme_preference_buttons(ui);
        });
        */

        egui::SidePanel::left(Id::new(0))
            .resizable(false)
            .default_width(160.0)
            .min_width(160.0)
            .show(ctx, |ui| {
                register_gadgets(self, ui);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("You have {} windows open.", self.instances.len()));

            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                ui.add(egui::github_link_file!(
                    "https://github.com/emilk/eframe_template/blob/main/",
                    "Source code."
                ));
                egui::warn_if_debug_build(ui);
            });

            for (id, open, widget) in &mut self.instances {
                widget.show(id, ctx, open);
            }
        });

        self.instances.retain(|(_, open, _)| *open);
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
