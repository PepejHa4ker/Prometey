use eframe::egui;
use eframe::egui::{Ui, Context};
use shared::settings::Settings;
pub mod misc_tab;
pub mod aim_tab;
pub mod loot_tab;

pub struct TabComponent {
    pub name: String,
    pub active: bool,
    pub tab: Box<dyn Tab>,
    pub group: TabGroup
}

pub trait Tab {

    fn render(&mut self, context: &Context, ui: &mut Ui, settings: &mut Settings);
}

pub enum TabGroup {
    Aim,
    Visuals,
    Loot,
    Misc,

}

