#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Create main window loading WhatsApp Web
            tauri::webview::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://web.whatsapp.com".parse().unwrap()),
            )
            .title("BetterWhatsApp")
            .inner_size(1200.0, 800.0)
            .build()?;

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {})
}
