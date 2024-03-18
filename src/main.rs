#![cfg(feature = "gui-native")]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    vrc_keyboard::run_gui_native().await
}
