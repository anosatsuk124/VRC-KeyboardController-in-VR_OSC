/* FIXME:
    #[cfg(all(feature = "gui-native", feature = "gui-wasm"))]
    compile_error!("feature gui-native and feature gui-wasm cannot be enabled at the same time");
*/

use crate::ui::{Canvas, CanvasPreference};
use anyhow::Result;
use eframe::egui;

const APP_TITLE: &str = "VRCKeyboard OSC";
const DEFAULT_LOCALE: &str = "en-US";
const ENV_ENV_LOGGER: &str = "RUST_LOG";

pub fn init_logger() -> Result<()> {
    #[cfg(not(feature = "gui"))]
    env_logger::init();

    #[cfg(feature = "gui")]
    if std::env::var(ENV_ENV_LOGGER).is_ok() {
        env_logger::init();
    } else {
        egui_logger::init()?;
    }

    Ok(())
}

/// Run the GUI application.
#[cfg(feature = "gui-native")]
pub async fn run_gui_native() -> Result<()> {
    init_logger()?;

    let system_locale = if let Some(locale) = sys_locale::get_locale() {
        crate::available_locales_wrapped()
            .iter()
            .find(|l| l.to_string() == locale)
            .map(|l| l.to_string())
            .unwrap_or(DEFAULT_LOCALE.to_string())
    } else {
        DEFAULT_LOCALE.to_string()
    };

    rust_i18n::set_locale(system_locale.as_str());
    log::info!("Set the system locale as: {}", system_locale);

    let mut options = eframe::NativeOptions::default();

    let initial_canvas_face = Canvas::CANVAS_SIZE_DEFAULT * CanvasPreference::ASPECT_RATIO_DEFAULT;
    options.initial_window_size = Some(egui::vec2(
        initial_canvas_face.x + Canvas::ACTIVE_RECT_MARGIN * 2.0,
        initial_canvas_face.y + Canvas::ACTIVE_RECT_MARGIN * 2.0,
    ));

    Err(anyhow::anyhow!(
        "Couldn't start with the reason why: {:?}",
        eframe::run_native(APP_TITLE, options, Box::new(|cc| Box::new(Canvas::new(cc))),)
    ))
}
