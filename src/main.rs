mod keybinds;
mod window;
mod window_manager;

use keybinds::{handle_hotkey, register_hotkeys, unregister_hotkeys};
use std::io::Error;
use std::ptr::null_mut;
use winapi::um::winuser::WM_HOTKEY;
use winapi::um::winuser::{GetMessageW, RegisterHotKey, UnregisterHotKey, MSG};
use window_manager::WindowManager;

fn main() -> Result<(), Error> {
    let mut leader_pressed = false;
    let mut window_manager = WindowManager::new();
    window_manager.set_windows();
    unregister_hotkeys();
    match register_hotkeys() {
        Ok(()) => println!("Hotkeys registerd!"),
        Err(e) => println!("Failed to registrer hotkeys: {}", e),
    }
    unsafe {
        let mut msg: MSG = Default::default();
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            println!("leader: {}", leader_pressed);
            if msg.message == WM_HOTKEY {
                leader_pressed =
                    handle_hotkey(msg.wParam as i32, &mut window_manager, leader_pressed);
            }
        }
    }
    unregister_hotkeys();
    Ok(())
}
