#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::fs;
use std::thread;
use tauri::{AppHandle, Manager, Window};
use tauri_plugin_wallpaper::Wallpaper;
use std::sync::Mutex;
use once_cell::sync::Lazy;


#[tauri::command]
fn read_store() -> String {
    let content = fs::read_to_string("../store.json")
        .expect("");
    content.into()
}

#[tauri::command]
fn write_store(content: String) {
    fs::write("../store.json", content)
        .expect("");
}

#[tauri::command]
fn wallpaper(app_handle: AppHandle) {
    let win = app_handle.get_window("main").unwrap();
    Wallpaper::attach(&(win));
}


#[tauri::command]
fn wallpaper_service() {
    thread::spawn(|| {
        extern crate winapi;
        use std::thread;
        use std::time::Duration;
        use std::ffi::{OsStr, OsString};
        use std::os::windows::ffi::{OsStrExt, OsStringExt};
        use winapi::shared::windef::HWND;
        use winapi::um::winuser::{FindWindowW, GetClassNameW, GetForegroundWindow, GetClassNameA};
        use std::sync::Mutex;


        fn send_to_js(wallpaper: String, audio: String) {
            let window = WALLPAPER_WINDOW.lock().unwrap();

            if let Some(window) = &*window {
                let send = format!("{}('{}', {})", "change", wallpaper, audio);
                window.eval(&send).unwrap();
            }
        }
        fn get_class_name(window_title: &str) -> Option<String> {
            let window_title = OsStr::new(window_title).encode_wide().chain(Some(0)).collect::<Vec<_>>();
            let hwnd = unsafe { FindWindowW(std::ptr::null(), window_title.as_ptr()) };
            if hwnd.is_null() {
                return None;
            }

            let mut class_name = vec![0u16; 256];
            let result = unsafe { GetClassNameW(hwnd as HWND, class_name.as_mut_ptr(), class_name.len() as i32) };
            if result == 0 {
                return None;
            }

            let class_name = OsString::from_wide(&class_name[..result as usize]).into_string().unwrap();
            Some(class_name)
        }


        lazy_static::lazy_static! {
            pub static ref WALLPAPER: Mutex<bool> = Mutex::new(true);
            pub static ref AUDIO: Mutex<bool> = Mutex::new(false);
        }


        fn is_window_maximized(hwnd: winapi::shared::windef::HWND) -> bool {
            let window_placement = unsafe {
                let mut placement = winapi::um::winuser::WINDOWPLACEMENT {
                    length: std::mem::size_of::<winapi::um::winuser::WINDOWPLACEMENT>() as u32,
                    ..std::mem::zeroed()
                };
                winapi::um::winuser::GetWindowPlacement(hwnd, &mut placement);
                placement
            };
            window_placement.showCmd == winapi::um::winuser::SW_MAXIMIZE as u32
        }

            
        unsafe extern "system" fn enum_windows_callback(hwnd: winapi::shared::windef::HWND, _l_param: winapi::shared::minwindef::LPARAM) -> winapi::shared::minwindef::BOOL {
            if is_window_maximized(hwnd) {
                let mut rect = winapi::shared::windef::RECT {
                    left: 0,
                    top: 0,
                    right: 0,
                    bottom: 0,
                };
                winapi::um::winuser::GetWindowRect(hwnd, &mut rect);
                if rect.left != 0 || rect.top != 0 {
                    let mut window_title = vec![0u16; 256];
                    let length = winapi::um::winuser::GetWindowTextW(hwnd, window_title.as_mut_ptr(), window_title.len() as i32);
                    let title = String::from_utf16_lossy(&window_title[..length as usize]);
                    if length > 0 {
                        if let Some(class_name) = get_class_name(&title) {
                            let mediaplayer = "Windows.UI.Core.CoreWindow";

                            if class_name != mediaplayer {
                                *WALLPAPER.lock().unwrap() = false;
                            }
                        }
                    }
                }
            }
            1 // Continue enumeration
        }

        fn audio() {
            let mut window_class: [i8; 256] = [0; 256];
            let current_window = unsafe { GetForegroundWindow() };
            let result = unsafe {
                GetClassNameA(current_window, window_class.as_mut_ptr(), window_class.len() as i32)
            };
            if result != 0 {
                let class_name = unsafe {
                    String::from_utf8_lossy(std::slice::from_raw_parts(
                        window_class.as_ptr() as *const u8,
                        result as usize
                    ))
                };
                let allowed: Vec<&str> = vec!["WorkerW", "Progman", "Shell_TrayWnd"];
                if allowed.contains(&&class_name[..]) {
                    *AUDIO.lock().unwrap() = true;
                }
                else {
                    *AUDIO.lock().unwrap() = false;
                }
            }
        }


        loop {
            unsafe {
                winapi::um::winuser::EnumWindows(Some(enum_windows_callback), 0);
                audio();

                let wallpaper = *WALLPAPER.lock().unwrap();
                let audio = *AUDIO.lock().unwrap();

                send_to_js(wallpaper.to_string(), audio.to_string());
                
                *WALLPAPER.lock().unwrap() = true;
                *AUDIO.lock().unwrap() = false;
            }
            thread::sleep(Duration::from_millis(100));
        }
    });
}




// Define the lazy static variable
static WALLPAPER_WINDOW: Lazy<Mutex<Option<Window>>> = Lazy::new(|| Mutex::new(None));

fn main() {
    tauri::Builder::default()
        .plugin(
            Wallpaper::init(),
        )
        .setup(|app| Ok({
            let wallpaper_window = app.get_window("wallpaper").unwrap();
            let mut global_window = WALLPAPER_WINDOW.lock().unwrap();
            *global_window = Some(wallpaper_window.clone());
            Wallpaper::attach(&(wallpaper_window));
        }))
        .invoke_handler(tauri::generate_handler![read_store, write_store, wallpaper, wallpaper_service])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
