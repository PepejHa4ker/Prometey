use eframe::egui::{Ui, Context, Widget, Slider, ComboBox};
use shared::settings::Settings;
use crate::app::app::layout;
use crate::app::tabs::Tab;
use crate::app::widgets::ToggleSwitch;

#[derive(Default)]
pub struct LootTab;



impl Tab for LootTab {

    fn render(&mut self, context: &Context, ui: &mut Ui, settings: &mut Settings) {
        ui.with_layout(layout(), |ui| {
            ui.columns(2, |columns| {
                let mut column = columns.get_mut(0).unwrap();
                column.vertical(|ui| {
                    ToggleSwitch::new(&mut settings.loot_enabled, "Loot").ui(ui);
                    ToggleSwitch::new(&mut settings.ammo_enabled, "Ammo").ui(ui);
                    ToggleSwitch::new(&mut settings.items_enabled, "Items").ui(ui);
                    ToggleSwitch::new(&mut settings.clothes_enabled, "Clothes").ui(ui);
                    ToggleSwitch::new(&mut settings.backpack_enabled, "Backpack").ui(ui);
                    ToggleSwitch::new(&mut settings.medicines_enabled, "Medicines").ui(ui);
                    ToggleSwitch::new(&mut settings.explosives_enabled, "Explosives").ui(ui);


                });
                let mut column = columns.get_mut(1).unwrap();

                column.vertical(|ui| {
                    ToggleSwitch::new(&mut settings.food_enabled, "Food").ui(ui);
                    ToggleSwitch::new(&mut settings.weapons_enabled, "Weapons").ui(ui);
                    ToggleSwitch::new(&mut settings.building_enabled, "Building").ui(ui);
                    ToggleSwitch::new(&mut settings.car_items_enabled, "Car items").ui(ui);
                    ToggleSwitch::new(&mut settings.attachments_enabled, "Attachments").ui(ui);
                    ToggleSwitch::new(&mut settings.consumables_enabled, "Consumables").ui(ui);
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Distance");
                        Slider::new(&mut settings.loot_distance, 0..=1000).ui(ui);
                    });

                });



            });


        });


    }
}