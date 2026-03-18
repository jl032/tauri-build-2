use tauri::Manager;

const INIT_SCRIPT: &str = r#"
    document.addEventListener('click', function(e) {
        var a = e.target.closest('a');
        if (a && a.href && !a.href.startsWith(window.location.origin)) {
            e.preventDefault();
            window.__TAURI__.shell.open(a.href);
        }
    });
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://feeder.co/reader".parse().unwrap())
            )
            .title("Feeder")
            .inner_size(1280.0, 800.0)
            .initialization_script(INIT_SCRIPT)
            .build()?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
