use tauri::{Manager, Emitter}; 
use std::time::Duration;
use std::thread; 
use std::process::{Command, Child};
use std::sync::Mutex;

#[cfg(target_os = "macos")]
use cocoa::base::{id, nil, YES, NO};
#[cfg(target_os = "macos")]
use objc::{msg_send, sel, sel_impl, class};
#[cfg(target_os = "macos")]
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

#[cfg(target_os = "macos")]
static WINDOW_PTR: AtomicUsize = AtomicUsize::new(0);
#[cfg(target_os = "macos")]
static CLICK_THROUGH: AtomicBool = AtomicBool::new(false);

struct BackendProcesses(Mutex<Vec<Child>>);

#[cfg(target_os = "macos")]
extern "C" fn can_become_key_window(_this: &objc::runtime::Object, _cmd: objc::runtime::Sel) -> bool {
    true
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn set_click_through(window: tauri::Window, ignore: bool) {
    #[cfg(target_os = "macos")]
    {
        CLICK_THROUGH.store(ignore, Ordering::SeqCst);
        let ptr = WINDOW_PTR.load(Ordering::SeqCst);
        if ptr != 0 {
            let ns_window = ptr as id;
            unsafe {
                let val = if ignore { YES } else { NO };
                let () = msg_send![ns_window, setIgnoresMouseEvents: val];
            }
        } else {
            let _ = window.set_ignore_cursor_events(ignore);
        }
    }
    #[cfg(not(target_os = "macos"))]
    {
        let _ = window.set_ignore_cursor_events(ignore);
    }
}

#[tauri::command]
fn open_new_window(app: tauri::AppHandle, url: String, label: String) {
    if let Ok(parsed_url) = url.parse() {
        let _ = tauri::WebviewWindowBuilder::new(
            &app,
            label,
            tauri::WebviewUrl::External(parsed_url)
        )
        .title("设置面板")
        .inner_size(900.0, 700.0)
        .transparent(false)
        .decorations(true)  
        .build();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, set_click_through, open_new_window])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let resource_dir = app.path().resource_dir().expect("无法获取资源目录");

            #[cfg(target_os = "macos")]
            let python_bin = resource_dir.join(".venv").join("bin").join("python3");
            #[cfg(target_os = "windows")]
            let python_bin = resource_dir.join(".venv").join("Scripts").join("python.exe");

            let scripts = vec!["memory_server.py", "main_server.py", "agent_server.py"];
            let mut children = Vec::new();

            for script in scripts {
                let script_path = resource_dir.join(script);
                println!("尝试启动: {:?}", script_path);

                match Command::new(&python_bin)
                    .arg(&script_path)
                    .current_dir(&resource_dir) 
                    .spawn() 
                {
                    Ok(child) => {
                        println!("成功启动后端服务: {}", script);
                        children.push(child);
                    }
                    Err(e) => eprintln!("启动后端服务失败 {}: {}", script, e),
                }
            }

            app.manage(BackendProcesses(Mutex::new(children)));

            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            let window = app.get_webview_window("main").unwrap();

            if let Ok(monitors) = window.available_monitors() {
                if !monitors.is_empty() {
                    let first_pos = monitors[0].position();
                    let first_size = monitors[0].size();
                    let mut min_x = first_pos.x;
                    let mut min_y = first_pos.y;
                    let mut max_x = first_pos.x + first_size.width as i32;
                    let mut max_y = first_pos.y + first_size.height as i32;

                    for m in &monitors[1..] {
                        let p = m.position();
                        let s = m.size();
                        min_x = min_x.min(p.x);
                        min_y = min_y.min(p.y);
                        max_x = max_x.max(p.x + s.width as i32);
                        max_y = max_y.max(p.y + s.height as i32);
                    }

                    let total_width = (max_x - min_x) as u32;
                    let total_height = (max_y - min_y) as u32;

                    let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x: min_x, y: min_y }));
                    let _ = window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width: total_width, height: total_height }));
                }
            }

            #[cfg(target_os = "macos")]
            {
                let app_handle = app.handle().clone();
                let window_clone = window.clone();

                thread::spawn(move || {
                    thread::sleep(Duration::from_millis(500));

                    let _ = app_handle.run_on_main_thread(move || {
                        if let Ok(ns_window_ptr) = window_clone.ns_window() {
                            let ns_window = ns_window_ptr as id;
                            unsafe {
                                WINDOW_PTR.store(ns_window as usize, Ordering::SeqCst);

                                extern "C" {
                                    fn object_setClass(obj: id, cls: id) -> id;
                                }

                                let class_name = "NekoPanel";
                                let mut cls = objc::runtime::Class::get(class_name);
                                if cls.is_none() {
                                    if let Some(mut decl) = objc::declare::ClassDecl::new(class_name, class!(NSPanel)) {
                                        decl.add_method(sel!(canBecomeKeyWindow), can_become_key_window as extern "C" fn(&objc::runtime::Object, objc::runtime::Sel) -> bool);
                                        cls = Some(decl.register());
                                    }
                                }

                                if let Some(custom_cls) = cls {
                                    object_setClass(ns_window, custom_cls as *const _ as id);
                                } else {
                                    object_setClass(ns_window, msg_send![class!(NSPanel), class]);
                                }

                                let current_style: u64 = msg_send![ns_window, styleMask];
                                let () = msg_send![ns_window, setStyleMask: current_style | 128]; 

                                let () = msg_send![ns_window, setCollectionBehavior: 273_u64];
                                let () = msg_send![ns_window, setLevel: 2147483630_i32];

                                let () = msg_send![ns_window, orderOut: nil];
                                let () = msg_send![ns_window, orderFrontRegardless];
                                
                                let ignore = CLICK_THROUGH.load(Ordering::SeqCst);
                                let val = if ignore { cocoa::base::YES } else { cocoa::base::NO };
                                let () = msg_send![ns_window, setIgnoresMouseEvents: val];
                            }
                        }
                    });
                });
            }

            let window_clone = window.clone();
            thread::spawn(move || {
                loop {
                    if let Ok(pos) = window_clone.cursor_position() {
                        let _ = window_clone.emit("neko_radar_tick", (pos.x, pos.y));
                    }
                    thread::sleep(Duration::from_millis(12));
                }
            });
            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building tauri application");

    builder.run(|app_handle, event| match event {
        tauri::RunEvent::Exit => {
            let state: tauri::State<BackendProcesses> = app_handle.state();
            let mut children = state.0.lock().unwrap();
            for child in children.iter_mut() {
                println!("正在关闭关联的 Python 进程: {}", child.id());
                let _ = child.kill();
            }
        }
        _ => {}
    });
}
