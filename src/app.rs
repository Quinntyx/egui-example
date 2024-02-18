use eframe::{egui, CreationContext};
use egui::{Align, Layout, FontId, Stroke, Rounding};
use egui::epaint::FontFamily;

use crate::model::context::Context;
use crate::gui::dynamic_button::{Draw, ResetCounterButton, SpawnNewWindowButton, DropDownButton};
use crate::gui::definitions::color;

pub struct App {
    ctx: Context,
    buttons: Vec<Box<dyn Draw>>,
}

impl App {
    pub fn new (x: i128) -> Self {
        Self {
            ctx: Context::new(x),
            buttons: vec![
                Box::new(ResetCounterButton{}),
                Box::new(SpawnNewWindowButton{}),
                Box::new(DropDownButton::new())
            ],
        }
    }

    pub fn setup (cc: &CreationContext) -> Box<dyn eframe::App> {
        let mut style = eframe::egui::Style::default();

        style.spacing.item_spacing = egui::vec2(12., 0.);
        style.spacing.button_padding = egui::vec2(18., 8.);

        // style.override_font_id = Some(FontId::new(32., FontFamily::Name("Consolas".into())));
        style.override_font_id = Some(FontId::new(32., FontFamily::Monospace));

        style.animation_time = 0.2;

        style.visuals.widgets.inactive.weak_bg_fill = color::BG_DARK;
        style.visuals.widgets.inactive.rounding = Rounding::default().at_least(50.);

        style.visuals.widgets.hovered.rounding = Rounding::default().at_least(50.);
        style.visuals.widgets.hovered.weak_bg_fill = color::BG_DARK;
        style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
        
        style.visuals.widgets.active.bg_stroke = Stroke::NONE;
        style.visuals.widgets.active.rounding = Rounding::default().at_least(50.);
        style.visuals.widgets.active.weak_bg_fill = color::BG_DARK;

        style.visuals.window_fill = color::BG_PRIMARY;
        style.visuals.panel_fill = color::BG_PRIMARY;

        cc.egui_ctx.set_style(style);

        Box::new(Self::new(0))
    }
}

impl eframe::App for App {
    fn update (&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let app_ctx = &mut self.ctx;

        app_ctx.update().expect("Update should not fail");

        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.set_height(60.);

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                for button in &self.buttons {
                    button.draw(app_ctx, ui);
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Counter");

            ui.horizontal(|ui| {
                let dec = ui.button("-1");
                let _label = ui.label(app_ctx.x.to_string());
                let inc = ui.button("+1");

                if inc.clicked() { app_ctx.x += 1 }
                if dec.clicked() { app_ctx.x -= 1 }
            });

            ui.code(&app_ctx.term_text);
        });
    }
}
