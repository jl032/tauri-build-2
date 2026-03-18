use tauri::Manager;
use tauri_plugin_shell::ShellExt;

const NAV_SCRIPT: &str = r#"
    (function() {
        if (document.getElementById('__tauri_nav__')) return;
        var nav = document.createElement('div');
        nav.id = '__tauri_nav__';
        nav.style.cssText = 'position:fixed;top:0;left:0;right:0;height:40px;background:#1a1a1a;display:flex;align-items:center;padding:0 8px;gap:8px;z-index:2147483647;';
        function btn(label, action) {
            var b = document.createElement('button');
            b.innerText = label;
            b.style.cssText = 'background:#333;color:#fff;border:none;border-radius:4px;padding:4px 10px;cursor:pointer;font-size:14px;flex-shrink:0;';
            b.onclick = action;
            return b;
        }
        var input = document.createElement('input');
        input.value = location.href;
        input.style.cssText = 'flex:1;background:#333;color:#fff;border:none;border-radius:4px;padding:4px 10px;font-size:13px;outline:none;';
        input.addEventListener('keydown', function(e) {
            if (e.key === 'Enter') {
                var url = input.value;
                if (!url.startsWith('http')) url = 'https://' + url;
                location.href = url;
            }
        });
        nav.appendChild(btn('\u2190', function() { history.back(); }));
        nav.appendChild(btn('\u2192', function() { history.forward(); }));
        nav.appendChild(btn('\u27f3', function() { location.reload(); }));
        nav.appendChild(input);
        document.body.style.paddingTop = '40px';
        document.body.appendChild(nav);
    })();
"#;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            let handle = app.handle().clone();
            let window = tauri::WebviewWindowBuilder::new(
                app,
                "main",
                tauri::WebviewUrl::External("https://z-library.sk/".parse().unwrap())
            )
            .title("Zlib")
            .inner_size(1280.0, 800.0)
            .initialization_script(NAV_SCRIPT)
            .build()?;

            window.on_navigation(move |url| {
                let url_str = url.to_string();
                if url_str.contains("z-library.sk") {
                    true
                } else {
                    let handle = handle.clone();
                    let url_str = url_str.clone();
                    tauri::async_runtime::spawn(async move {
                        let _ = handle.shell().open(&url_str, None);
                    });
                    false
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
