mod keybinds;
mod window;
mod window_manager;

use keybinds::{handle_hotkey, register_leader, unregister_leader};
use std::io::Error;
use std::ptr::null_mut;
use winapi::um::winuser::WM_HOTKEY;
use winapi::um::winuser::{GetMessageW, MSG};
use window_manager::WindowManager;

fn main() -> Result<(), Error> {
    let mut leader_pressed = false;
    let mut window_manager = WindowManager::new();
    window_manager.set_windows();
    unregister_leader();
    match register_leader() {
        Ok(()) => println!("Leader Registered"),
        Err(e) => println!("Failed to registrer leader: {}", e),
    }
    unsafe {
        let mut msg: MSG = Default::default();
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            if msg.message == WM_HOTKEY {
                match handle_hotkey(msg.wParam as i32, &mut window_manager, leader_pressed) {
                    Ok(leader) => leader_pressed = leader,
                    Err(e) => {
                        println!("Error {}", e);
                        break;
                    }
                }
            }
        }
    }
    unregister_leader();
    Ok(())
}

