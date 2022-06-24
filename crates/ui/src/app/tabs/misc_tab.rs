use eframe::egui::{Ui, Context, Widget, Slider};
use shared::settings::Settings;
use crate::app::app::layout;
use crate::app::tabs::Tab;
use crate::app::widgets::ToggleSwitch;

#[derive(Default)]
pub struct MiscTab;


impl Tab for MiscTab {
    fn render(&mut self, context: &Context, ui: &mut Ui, settings: &mut Settings) {
        ui.with_layout(layout(), |ui| {
            ui.columns(2, |columns| {
                let mut column = columns.get_mut(0).unwrap();
                column.vertical(|ui| {
                    ToggleSwitch::new(&mut settings.noclip_enabled, "NoClip")
                        .ui(ui);
                    ToggleSwitch::new(&mut settings.debug_camera, "Debug Camera")
                        .ui(ui);
                    ToggleSwitch::new(&mut settings.remove_grass, "Remove Grass")
                        .ui(ui);
                    ToggleSwitch::new(&mut settings.recoil_control, "Recoil Control")
                        .ui(ui);
                    ToggleSwitch::new(&mut settings.anti_spread, "Anti Spread")
                        .ui(ui);
                });
                let mut column = columns.get_mut(1).unwrap();

                column.vertical(|ui| {
                    ui.label("NoClip range");
                    Slider::new(&mut settings.noclip_range, 0..=1100).ui(ui);
                    ui.label("Camera Speed");
                    Slider::new(&mut settings.camera_speed, 0.100..=0.250).ui(ui);
                });


            });
        });
    }
}