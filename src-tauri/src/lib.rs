use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.on_navigation(|_url| {
                true
            });
            Ok(())
        })
        .on_webview_event(|window, event| {
            if let tauri::WebviewEvent::Navigation { url: _ } = event {
                window.eval("
                    document.addEventListener('click', (e) => {
                        const a = e.target.closest('a');
                        if (a && a.href && !a.href.startsWith(location.origin)) {
                            e.preventDefault();
                            if (window.__TAURI__ && window.__TAURI__.shell) {
                                window.__TAURI__.shell.open(a.href);
                            }
                        }
                    });
                ").unwrap();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
