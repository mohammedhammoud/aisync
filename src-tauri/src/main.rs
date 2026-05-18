// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    if std::env::args().any(|arg| arg == "--export-bindings") {
        aisync_lib::export_bindings();
        return;
    }

    aisync_lib::run()
}
