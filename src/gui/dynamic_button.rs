use eframe::egui::{Response, Ui, Button};

use crate::model::context::Context;
use crate::gui::definitions::font;

pub trait Draw {
    fn draw (&self, ctx: &mut Context, ui: &mut Ui) -> Response;
}

pub struct ResetCounterButton;

impl Draw for ResetCounterButton {
    fn draw (&self, ctx: &mut Context, ui: &mut Ui) -> Response {
        let button = ui.button(font::fg_mono_rt("Reset"));
        if button.clicked() {
            ctx.x = 0
        };
        button
    }
}

pub struct SpawnNewWindowButton;

impl Draw for SpawnNewWindowButton {
    fn draw (&self, _ctx: &mut Context, ui: &mut Ui) -> Response {
        let button = ui.add(Button::new(font::fg_mono_rt("New")).rounding(50.));
        if button.clicked() {
            println!("Sorry, this doesn't work yet :P");
        }

        button
    }
}

pub struct DropDownButton {
    pub options: Vec<String>
}

impl DropDownButton {
    pub fn new () -> Self {
        Self {
            options: vec!["cargo run".to_owned(), "cargo build".to_owned()]
        }
    }
}

impl Draw for DropDownButton {
    fn draw (&self, ctx: &mut Context, ui: &mut Ui) -> Response {
        ui.menu_button(font::fg_mono_rt(&self.options[ctx.dropdown_selected]), |ui| {
            for (idx, option) in self.options.iter().enumerate() {
                if option == &self.options[ctx.dropdown_selected] { continue; }
                if ui.button(option).clicked() {
                    ctx.dropdown_selected = idx;
                }
            }
        }).response
    }
}
