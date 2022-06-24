use eframe::egui::{Ui, Context, Widget, Slider};
use shared::settings::Settings;
use crate::app::app::layout;
use crate::app::tabs::Tab;
use crate::app::widgets::ToggleSwitch;

#[derive(Default)]
pub struct AimTab;

impl Tab for AimTab {
    fn render(&mut self, context: &Context, ui: &mut Ui, settings: &mut Settings) {
        ui.with_layout(layout(), |ui| {
            ui.columns(2, |columns| {
                let mut column = columns.get_mut(0).unwrap();

                column.vertical(|ui| {
                    let enabled = ToggleSwitch::new(&mut settings.silent_enabled, "Silent").ui(ui);
                    let draw_fov = ToggleSwitch::new(&mut settings.silent_draw_fov, "Draw FOV").ui(ui);
                    let on_players = ToggleSwitch::new(&mut settings.silent_players, "On Players").ui(ui);
                    let on_zombies = ToggleSwitch::new(&mut settings.silent_zombies, "On Zombies").ui(ui);
                });
                let mut column = columns.get_mut(1).unwrap();
                column.vertical(|ui| {
                    ui.label("FOV");
                    let fov = Slider::new(&mut settings.silent_fov, 1..=180).ui(ui);
                    ui.label("Distance");
                    let dist = Slider::new(&mut settings.silent_dist, 1..=1100).ui(ui);
                });
            })
        });
    }
}