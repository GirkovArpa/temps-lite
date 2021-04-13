// hide console window on Windows
#![windows_subsystem="windows"]
#[macro_use] extern crate sciter;

#[cfg(windows)]
use winreg;

struct EventHandler;
#[cfg(windows)]
impl EventHandler {
    #[allow(non_snake_case)]
    fn createWindowsShortcut(&self, add: bool) -> sciter::Value {
        // https://users.rust-lang.org/t/how-to-make-my-exe-autorun-in-windows/49045/12
        use std::path::Path;
        use winreg::enums::*;
        use winreg::RegKey;
        let hkcu = RegKey::predef(HKEY_CURRENT_USER);
        let path = Path::new("Software").join("Microsoft").join("Windows").join("CurrentVersion").join("Run");
        let (key, disp) = hkcu.create_subkey(&path).unwrap();
        dbg!(&disp);
        let path = format!("\"{}\"", std::env::current_exe().unwrap().to_str().unwrap().to_string());
        if add {
            key.set_value("temps-lite", &path).unwrap();
        } else {
            key.delete_value("temps-lite").unwrap();
        }
        sciter::Value::from(true)
    }
}

impl sciter::EventHandler for EventHandler {
    #[cfg(windows)]
    dispatch_script_call! (
        fn createWindowsShortcut(bool);
    );
}

fn main() {
    // allows CTRL+SHIFT+I to connect to inspector.exe
    sciter::set_options(sciter::RuntimeOptions::DebugMode(false)).unwrap();
    let archived = include_bytes!("../target/assets.rc");
    sciter::set_options(sciter::RuntimeOptions::ScriptFeatures(
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SYSINFO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_FILE_IO  as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_EVAL     as u8 |
        sciter::SCRIPT_RUNTIME_FEATURES::ALLOW_SOCKET_IO as u8 
    )).unwrap();
    let mut frame = sciter::Window::new();
    frame.event_handler(EventHandler {});
    frame.archive_handler(archived).unwrap();
    frame.load_file("this://app/main.html");
    frame.run_app();
}
