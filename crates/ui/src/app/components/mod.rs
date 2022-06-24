pub mod tab_selector;

use eframe::egui;
use egui::Ui;
use super::tabs;

pub trait Component {

    fn render(&mut self, context: &egui::Context, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui));

}
