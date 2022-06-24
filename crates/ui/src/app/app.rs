use std::thread::sleep;
use std::time::Duration;
use eframe;
use eframe::egui::{Context, vec2, CentralPanel, Direction, Align, Layout, Vec2, Visuals, Rgba, TopBottomPanel};
use eframe::egui::CursorIcon::Text;
use eframe::egui::menu;
use eframe::egui::Shape::{LineSegment, Mesh};
use eframe::epaint::{CircleShape, RectShape, TextShape};
use eframe::{Frame, Storage};
use shared::is_menu_open;
use crate::app::components::tab_selector::*;
use crate::app::components::Component;


pub struct App {
    tab_selector: TabSelectorComponent,

}

impl App {
    pub fn new() -> Self {

        App {
            tab_selector: TabSelectorComponent::new(),
        }
    }


    pub fn render(&mut self, ctx: &Context) {
        if !shared::is_menu_open() {
            return;
        }
        let mut style = (*ctx.style()).clone();
        style.spacing.item_spacing = vec2(15.0, 8.0);
        style.spacing.indent = 25.0;
        style.animation_time = 1.0/15.0;
        style.visuals.dark_mode = false;

        ctx.set_style(style);


        TopBottomPanel::top("Prometey").show(ctx, |ui| {
            ui.label("Prometey");
            ui.separator();
            self.tab_selector.render(ctx, ui,  |_| {});
        });
        sleep(Duration::from_millis(25));
    }
}


pub fn layout() -> Layout {
    Layout::from_main_dir_and_cross_align(Direction::TopDown, Align::Center)
}

// impl eframe::App for App {
//     fn clear_color(&self, _visuals: &Visuals) -> Rgba {
//         eframe::egui::Color32::from_rgba_unmultiplied(0, 0, 0, 0).into()
//     }
//
//     fn update(&mut self, ctx: &Context, frame: &mut Frame) {
//         if !is_menu_open() {
//             return;
//         }
//         ctx.request_repaint();
//         let mut style = (*ctx.style()).clone();
//         style.spacing.item_spacing = vec2(15.0, 8.0);
//         style.spacing.indent = 25.0;
//         style.animation_time = 1.0/15.0;
//
//         ctx.set_style(style);
//
//
//         TopBottomPanel::top("Prometey").show(ctx, |ui| {
//             ui.label("Prometey");
//             ui.separator();
//             self.tab_selector.render(ctx, ui,  |_| {});
//         });
//         sleep(Duration::from_millis(25));
//
//     }
// }
