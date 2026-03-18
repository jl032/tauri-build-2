#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let window = tauri::Manager::get_webview_window(app, "main").unwrap();
            window.eval("
                document.addEventListener('click', (e) => {
                    const a = e.target.closest('a');
                    if (a && a.href && !a.href.startsWith(location.origin)) {
                        e.preventDefault();
                        window.__TAURI__.shell.open(a.href);
                    }
                });
            ").unwrap();
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
