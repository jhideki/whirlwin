mod callbacks;
mod keybinds;
mod window;
mod window_manager;

use callbacks::win_event_proc;
use keybinds::{handle_hotkey, register_leader, unregister_leader};
use lazy_static::lazy_static;
use std::io::Error;
use std::ptr::{self, null_mut};
use std::sync::{Arc, Mutex};
use winapi::um::winuser::{GetMessageW, SetWinEventHook, EVENT_SYSTEM_FOREGROUND, MSG};
use winapi::um::winuser::{WINEVENT_OUTOFCONTEXT, WM_HOTKEY};
use window_manager::WindowManager;

lazy_static! {
    static ref LEADER_PRESSED: Arc<Mutex<bool>> = Arc::new((Mutex::new(false)));
}
fn main() -> Result<(), Error> {
    let mut window_manager = WindowManager::new();
    window_manager.set_windows();
    unregister_leader();
    match register_leader() {
        Ok(()) => println!("Leader Registered"),
        Err(e) => println!("Failed to registrer leader: {}", e),
    }
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            ptr::null_mut(),
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
    }
    unsafe {
        let mut msg: MSG = Default::default();
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            if msg.message == WM_HOTKEY {
                if let Ok(mut gaurd) = LEADER_PRESSED.lock() {
                    println!("{}", gaurd);
                    match handle_hotkey(msg.wParam as i32, &mut window_manager, *gaurd) {
                        Ok(leader) => {
                            *gaurd = leader;
                        }
                        Err(e) => {
                            println!("Error {}", e);
                            break;
                        }
                    }
                }
            }
        }
    }
    unregister_leader();
    Ok(())
}

