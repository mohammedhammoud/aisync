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
            core::config::get_defaults,
            core::config::get_globals,
            core::config::get_configs,
            core::skills::get_skills,
            core::config::get_config,
            core::config::create_config,
            core::config::update_config,
            core::config::delete_config,
            core::skills::get_skill,
            core::skills::create_skill,
            core::skills::update_skill,
            core::skills::delete_skill,
            core::instructions::read_instructions,
            core::instructions::write_instructions,
            core::github::auth::start_github_login,
            core::github::auth::logout_github,
            core::github::auth::get_github_sync_status,
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
    export_bindings();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            core::github::events::init(app.handle().clone());
            Ok(())
        })
        .invoke_handler(builder.invoke_handler())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
