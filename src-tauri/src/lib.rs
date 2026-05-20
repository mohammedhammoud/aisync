#[cfg(not(target_os = "macos"))]
compile_error!("AISync currently supports macOS only. Linux/Windows platform stubs are kept for future support.");

mod core;
mod os;
mod platform;

use specta_typescript::Typescript;
use tauri_specta::{collect_commands, Builder, ErrorHandlingMode};

type SpectaBuilder = Builder<tauri::Wry>;

#[cfg(test)]
mod test_support;

pub fn specta_builder() -> SpectaBuilder {
    Builder::<tauri::Wry>::new()
        .error_handling(ErrorHandlingMode::Throw)
        .commands(collect_commands![
            core::config::commands::get_defaults,
            core::config::commands::get_globals,
            core::config::commands::get_configs,
            core::skills::commands::get_skills,
            core::config::commands::get_config,
            core::config::commands::create_config,
            core::config::commands::update_config,
            core::config::commands::delete_config,
            core::skills::commands::get_skill,
            core::skills::commands::create_skill,
            core::skills::commands::update_skill,
            core::skills::commands::delete_skill,
            core::sync::commands::get_link_status,
            core::sync::commands::force_link_target,
            core::instructions::read_instructions,
            core::instructions::write_instructions,
            core::github::auth::commands::start_github_login,
            core::github::auth::commands::logout_github,
            core::github::auth::commands::get_github_sync_status,
            core::github::commands::setup_github_sync,
            core::github::commands::resolve_sync_conflict,
            core::github::commands::sync_github_now
        ])
}

pub fn export_bindings() {
    specta_builder()
        .export(Typescript::default(), "../src/base/tauri/bindings.ts")
        .expect("failed to export tauri typescript bindings");
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = specta_builder();

    #[cfg(debug_assertions)]
    if std::env::var_os("AISYNC_EXPORT_BINDINGS_ON_STARTUP").is_some() {
        export_bindings();
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            core::github::events::init(app.handle().clone());
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
