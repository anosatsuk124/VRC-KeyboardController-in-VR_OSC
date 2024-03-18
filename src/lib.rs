#[cfg(feature = "gui")]
pub mod app;
#[cfg(feature = "core")]
pub mod input;
#[cfg(feature = "core")]
pub mod osc;
#[cfg(feature = "gui")]
pub mod ui;

#[cfg(feature = "gui")]
pub use app::*;

rust_i18n::i18n!("locales", fallback = "en-US");

/// Top-level function wrapper.
pub fn available_locales_wrapped() -> &'static [&'static str] {
    available_locales()
}
