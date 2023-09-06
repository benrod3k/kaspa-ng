use crate::imports::*;

pub struct Settings {
    sender : Sender<Events>,

}

impl Settings {
    pub fn new(sender : Sender<Events>) -> Self {
        Self {
            sender,
        }
    }
}


impl SectionT for Settings {
    fn render(&mut self, _wallet : &mut Wallet, _ctx: &egui::Context, _frame: &mut eframe::Frame, ui : &mut egui::Ui) {
        ui.heading("Settings");
        ui.separator();
        ui.label("This is the settings page");
    }
}