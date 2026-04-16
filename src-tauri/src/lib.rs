mod commands;
mod detector;
mod optimizer;
mod profiles;
mod snapshot;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::detect_hardware,
            commands::get_profiles,
            commands::check_admin,
            commands::check_snapshot,
            commands::apply_profile,
            commands::revert_snapshot,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
