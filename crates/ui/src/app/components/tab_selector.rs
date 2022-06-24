use crate::app::components::Component;
use eframe::egui::{Ui, menu, Context, Widget, Slider};
use shared::settings;
use crate::Inverse;
use crate::app::widgets::*;
use crate::app::app::layout;
use crate::app::tabs::{Tab, TabComponent, TabGroup};
use crate::app::tabs::aim_tab::AimTab;
use crate::app::tabs::loot_tab::LootTab;
use crate::app::tabs::misc_tab::MiscTab;

pub struct TabSelectorComponent {
    tabs: [TabComponent; 4],
    active_tab: usize,
}


impl TabSelectorComponent {
    pub fn new() -> Self {
        TabSelectorComponent {
            tabs: [
                TabComponent { name: "Aim".to_string(), active: true, tab: Box::new(MiscTab), group: TabGroup::Aim },
                TabComponent { name: "Loot".to_string(), active: false, tab:  Box::new(MiscTab), group: TabGroup::Loot },
                TabComponent { name: "Visuals".to_string(), active: false, tab:  Box::new(MiscTab), group: TabGroup::Visuals },
                TabComponent { name: "Misc".to_string(), active: false, tab:  Box::new(MiscTab), group: TabGroup::Misc },
            ],
            active_tab: 0,
        }
    }
}


impl Component for TabComponent {
    fn render(&mut self, context: &Context, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        match self.group {
            TabGroup::Misc => {
                MiscTab::default().render(context, ui, settings());
            },
            TabGroup::Aim => {
                AimTab::default().render(context, ui, settings());
            },
            TabGroup::Loot => {
                LootTab::default().render(context, ui, settings());
            }

            _ => {}
        }
    }
}

impl Component for TabSelectorComponent {
    fn render(&mut self, context: &Context, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        menu::bar(ui, |ui| {
            for (idx, tab) in self.tabs.iter_mut().enumerate() {
                let label = ui.selectable_label(tab.active, &tab.name);
                if idx != 3 {
                    ui.separator();
                }
                if idx != self.active_tab {
                    tab.active = false
                }
                if label.clicked() {
                    if idx == self.active_tab {
                        continue;
                    }
                    self.active_tab = idx;
                    tab.active.inverse();
                }
            }
        });

        for tab in self.tabs.iter_mut(){
            if tab.active {
                tab.render(context, ui, |_| {});
            }
        }
    }
}