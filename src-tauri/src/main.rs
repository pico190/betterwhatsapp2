#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tauri::{Manager, WindowEvent, WebviewUrl, WebviewWindowBuilder};
use tauri::menu::{Menu, MenuItem};
use tauri::tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent};

fn main() {
    // Force XWayland instead of native Wayland for better KDE compatibility
    // This fixes the window decoration buttons not working after show/hide
    #[cfg(target_os = "linux")]
    unsafe {
        std::env::set_var("GDK_BACKEND", "x11");
    }
    
    tauri::Builder::default()
        .register_uri_scheme_protocol("better-whatsapp", |ctx, request| {
            let app = ctx.app_handle();
            if request.uri().path() == "/toggle_devtools" {
                 if let Some(window) = app.get_webview_window("main") {
                    if window.is_devtools_open() {
                        window.close_devtools();
                    } else {
                        window.open_devtools();
                    }
                }
            }
            tauri::http::Response::builder()
                .header("Access-Control-Allow-Origin", "*")
                .body(Vec::new())
                .unwrap()
        })
        .setup(|app| {
            // Inject CSS and spoof platform
            let css_content = include_str!("style.css");
            let css_escaped = css_content.replace("`", "\\`").replace("${", "\\${");

            let init_script = format!(r#"
                // Spoof Navigator
                Object.defineProperty(navigator, 'userAgent', {{
                    get: function () {{ return 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 BetterWhatsApp'; }}
                }});
                Object.defineProperty(navigator, 'platform', {{
                    get: function () {{ return 'Linux x86_64'; }}
                }});
                Object.defineProperty(navigator, 'vendor', {{
                    get: function () {{ return 'Google Inc.'; }}
                }});

                // Inject CSS
                const css = `{}`;
                const style = document.createElement('style');
                style.textContent = css;

                function inject() {{
                    if (document.head) {{
                        if (!document.head.contains(style)) {{
                            document.head.appendChild(style);
                        }}
                    }} else {{
                        setTimeout(inject, 10);
                    }}
                }}
                
                // Try to inject immediately and on load
                inject();
                window.addEventListener('DOMContentLoaded', inject);
                window.addEventListener('load', inject);
                
                // Observer to ensure it stays
                const observer = new MutationObserver(() => {{
                    inject();
                }});
                observer.observe(document.documentElement, {{ childList: true, subtree: true }});

                // Shortcut for DevTools
                window.addEventListener('keydown', (e) => {{
                    if (e.key === 'F12') {{
                        fetch('better-whatsapp://localhost/toggle_devtools');
                    }}
                }});

                // Zoom shortcuts
                window.addEventListener('keydown', (e) => {{
                    if (e.ctrlKey) {{
                        if (e.key === '=' || e.key === '+') {{
                            e.preventDefault();
                            let current = parseFloat(document.body.style.zoom) || 1;
                            document.body.style.zoom = current + 0.1;
                        }} else if (e.key === '-') {{
                            e.preventDefault();
                            let current = parseFloat(document.body.style.zoom) || 1;
                            document.body.style.zoom = current - 0.1;
                        }} else if (e.key === '0') {{
                            e.preventDefault();
                            document.body.style.zoom = 1;
                        }}
                    }}
                }});
            "#, css_escaped);

            let win_builder = WebviewWindowBuilder::new(
                app,
                "main",
                WebviewUrl::External("https://web.whatsapp.com".parse().unwrap())
            )
            .title("BetterWhatsApp")
            .inner_size(1200.0, 800.0)
            .decorations(true)
            .resizable(true)
            .maximizable(true)
            .minimizable(true)
            .user_agent("Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 BetterWhatsApp")
            .initialization_script(&init_script);

            let window = win_builder.build().unwrap();
            
            // Set window icon for Wayland/X11
            if let Some(icon) = app.default_window_icon() {
                let _ = window.set_icon(icon.clone());
            }

            // Track if window is hidden (not just minimized)
            let is_hidden = Arc::new(AtomicBool::new(false));
            let is_hidden_for_menu = is_hidden.clone();
            let is_hidden_for_tray = is_hidden.clone();
            let is_hidden_for_close = is_hidden.clone();

            // Setup Tray
            let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let toggle_i = MenuItem::with_id(app, "toggle", "Minimize", true, None::<&str>)?;
            let toggle_ref = Arc::new(toggle_i);
            let toggle_for_menu = toggle_ref.clone();
            let toggle_for_tray = toggle_ref.clone();
            let toggle_for_close = toggle_ref.clone();
            
            let tray_menu = Menu::with_items(app, &[toggle_for_menu.as_ref(), &quit_i])?;

            let _tray = TrayIconBuilder::with_id("tray")
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&tray_menu)
                .show_menu_on_left_click(false)
                .on_menu_event(move |app, event| {
                    match event.id.as_ref() {
                        "quit" => {
                            app.exit(0);
                        }
                        "toggle" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let hidden = is_hidden_for_menu.load(Ordering::SeqCst);
                                
                                if hidden {
                                    // Show window
                                    let _ = window.show();
                                    let _ = window.unminimize();
                                    let _ = window.set_focus();
                                    is_hidden_for_menu.store(false, Ordering::SeqCst);
                                    let _ = toggle_for_menu.set_text("Minimize");
                                } else {
                                    // Hide window
                                    let _ = window.hide();
                                    is_hidden_for_menu.store(true, Ordering::SeqCst);
                                    let _ = toggle_for_menu.set_text("Show");
                                }
                            }
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(move |tray, event| {
                    let app = tray.app_handle();
                    
                    match event {
                        TrayIconEvent::Click { button, button_state, .. } => {
                            if button_state != MouseButtonState::Up {
                                return;
                            }
                            
                            match button {
                                MouseButton::Left => {
                                    // Left click: toggle show/hide
                                    if let Some(window) = app.get_webview_window("main") {
                                        let hidden = is_hidden_for_tray.load(Ordering::SeqCst);
                                        
                                        if hidden {
                                            // Show window
                                            let _ = window.show();
                                            let _ = window.unminimize();
                                            let _ = window.set_focus();
                                            is_hidden_for_tray.store(false, Ordering::SeqCst);
                                            let _ = toggle_for_tray.set_text("Minimize");
                                        } else {
                                            // Hide window
                                            let _ = window.hide();
                                            is_hidden_for_tray.store(true, Ordering::SeqCst);
                                            let _ = toggle_for_tray.set_text("Show");
                                        }
                                    }
                                }
                                MouseButton::Right => {
                                    // Right click: menu opens automatically
                                }
                                _ => {}
                            }
                        }
                        _ => {}
                    }
                })
                .build(app)?;
            
            // Store is_hidden and toggle_item in app state for close event
            app.manage(is_hidden_for_close);
            app.manage(toggle_for_close);

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // Hide instead of close - keep process running for notifications
                let _ = window.hide();
                
                // Update hidden state
                if let Some(is_hidden) = window.app_handle().try_state::<Arc<AtomicBool>>() {
                    is_hidden.store(true, Ordering::SeqCst);
                }
                
                // Update menu text
                if let Some(toggle) = window.app_handle().try_state::<Arc<MenuItem<tauri::Wry>>>() {
                    let _ = toggle.set_text("Show");
                }
                
                api.prevent_close();
            }
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application")
        .run(|_app, _event| {})
}
