mod emoji;
mod puzzle_hunt_tools;

use emoji::EmojiPicker;
use puzzle_hunt_tools::PuzzleHuntTools;

use crate::MyApp;

pub trait HouseGadget {
    fn new() -> Self
    where
        Self: Sized;

    fn title(&self) -> String;

    fn ui(&mut self, ui: &mut egui::Ui);

    fn show(&mut self, id: &egui::Id, ctx: &egui::Context, open: &mut bool) {
        egui::Window::new(self.title())
            .id(*id)
            .open(open)
            .vscroll(true)
            .hscroll(true)
            .show(ctx, |ui| self.ui(ui));
    }
}

pub fn register_gadgets(app: &mut MyApp, ui: &mut egui::Ui) {
    if ui.button("Emoji picker").clicked() {
        let widget = EmojiPicker::new();
        let id = app.new_id();
        app.instances.push((id, true, Box::new(widget)));
    }
    if ui.button("Puzzle hunt tools").clicked() {
        let widget = PuzzleHuntTools::new();
        let id = app.new_id();
        app.instances.push((id, true, Box::new(widget)));
    }
}
