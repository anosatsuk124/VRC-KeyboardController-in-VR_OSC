use std::{borrow::BorrowMut, ops::Deref};

use anyhow::Result;
use eframe::{
    egui::{self, FontData, FontDefinitions, Key},
    epaint::FontFamily,
};
use rust_i18n::t;

use crate::{
    input::{self, INPUT_HANDLER},
    osc,
};

pub struct Canvas {
    canvas_size: f32,
    active_rect: egui::Rect,
    input_handler: Option<input::InputHandler>,
    osc_started: bool,
    preference: CanvasPreference,
}

impl Default for Canvas {
    fn default() -> Self {
        let pos = egui::Pos2::default();

        let preference = CanvasPreference::default();

        Self {
            canvas_size: Self::CANVAS_SIZE_DEFAULT,
            active_rect: egui::Rect::from_min_size(
                pos + egui::vec2(Self::ACTIVE_RECT_MARGIN, Self::ACTIVE_RECT_MARGIN),
                egui::vec2(
                    preference.aspect_ratio.x * Self::CANVAS_SIZE_DEFAULT,
                    preference.aspect_ratio.y * Self::CANVAS_SIZE_DEFAULT,
                ),
            ),
            input_handler: None,
            osc_started: false,
            preference,
        }
    }
}

pub struct CanvasPreference {
    aspect_ratio: egui::Vec2,
    zoom_ratio: f32,
}

impl Default for CanvasPreference {
    fn default() -> Self {
        Self {
            aspect_ratio: Self::ASPECT_RATIO_DEFAULT,
            zoom_ratio: Self::ZOOM_RATIO_DEFAULT,
        }
    }
}

impl CanvasPreference {
    pub const ZOOM_RATIO_DEFAULT: f32 = 2.0;
    pub const ASPECT_RATIO_DEFAULT: egui::Vec2 = egui::vec2(16.0, 9.0);
}

impl Canvas {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut fonts = FontDefinitions::default();

        const NOTO_SANS: &[u8] = include_bytes!("../fonts/NotoSansJP-Regular.ttf");
        const DEFAULT_FONT_NAME: &str = "Noto Sans JP";

        let noto_sans = FontData::from_static(NOTO_SANS);
        fonts
            .font_data
            .insert(DEFAULT_FONT_NAME.to_owned(), noto_sans);

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, DEFAULT_FONT_NAME.to_owned());

        cc.egui_ctx.set_fonts(fonts);

        if let Some(_storage) = cc.storage {
            // TODO: load preference from storage
        }

        Default::default()
    }

    pub const ACTIVE_RECT_MARGIN: f32 = 50.0;

    pub const CANVAS_SIZE_DEFAULT: f32 = 50.0;

    const DEFAULT_POS: egui::Pos2 = egui::pos2(0f32, 0f32);

    fn init_active_rect(&self, pos: Option<egui::Pos2>) -> egui::Rect {
        match pos {
            Some(p) => egui::Rect::from_min_size(
                p + egui::vec2(Self::ACTIVE_RECT_MARGIN, Self::ACTIVE_RECT_MARGIN),
                self.canvas_face(),
            ),
            None => egui::Rect::from_min_size(
                Self::DEFAULT_POS + egui::vec2(Self::ACTIVE_RECT_MARGIN, Self::ACTIVE_RECT_MARGIN),
                self.canvas_face(),
            ),
        }
    }

    fn canvas_face(&self) -> egui::Vec2 {
        self.preference.aspect_ratio * self.canvas_size
    }

    fn update_window_size(&mut self, frame: &mut eframe::Frame) {
        let canvas_face = self.canvas_face();

        frame.set_window_size(egui::vec2(
            canvas_face.x + Self::ACTIVE_RECT_MARGIN * 2.0,
            canvas_face.y + Self::ACTIVE_RECT_MARGIN * 2.0,
        ));
    }

    pub fn from_absolute_to_relative(&self, pos: egui::Pos2) -> Option<egui::Pos2> {
        let relative = (pos - self.active_rect.min).to_pos2();

        if relative.x < 0.0 || relative.y < 0.0 {
            return None;
        }

        Some(relative)
    }
}

fn get_interact_pos(input_state: &egui::InputState) -> Option<egui::Pos2> {
    let pointer = &input_state.pointer;
    let is_down = pointer.any_down();
    let is_moving = pointer.is_moving();

    if !is_down {
        return None;
    }

    pointer.interact_pos()
}

impl eframe::App for Canvas {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(input_handler) = INPUT_HANDLER.get() {
                if let Ok(mut input_handler) = input_handler.lock() {
                    ui.input(|i| {
                        if i.key_down(Key::ArrowRight) {
                            input_handler.look_right();
                        }
                        if i.key_down(Key::ArrowLeft) {
                            input_handler.look_left();
                        }

                        if i.key_down(Key::W) {
                            input_handler.mov_forward();
                        }
                        if i.key_down(Key::A) {
                            input_handler.mov_left();
                        }
                        if i.key_down(Key::S) {
                            input_handler.mov_backward();
                        }
                        if i.key_down(Key::D) {
                            input_handler.mov_right();
                        }

                        input_handler.eval().unwrap();
                    });
                }
            };

            ui.menu_button(t!("Logs"), |ui| {
                egui_logger::logger_ui(ui);
            });

            if ui.button(t!("Start")).clicked() && !self.osc_started {
                self.osc_started = true;

                if let Err(e) = osc::start_osc() {
                    log::error!("Failed to start osc: {}", e);
                    self.osc_started = false;
                }
            }
        });
    }
}
