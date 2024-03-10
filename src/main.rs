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
use winapi::um::winuser::GetForegroundWindow;
use winapi::um::winuser::PM_REMOVE;
use winapi::um::winuser::{PeekMessageW, SetWinEventHook, EVENT_SYSTEM_FOREGROUND, MSG};
use winapi::um::winuser::{WINEVENT_OUTOFCONTEXT, WM_HOTKEY};
use window::Window;
use window_manager::WindowManager;

//I couldn't think of a better way to signal the window manager from the event hook
lazy_static! {
    static ref LEADER_PRESSED: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
    static ref NEW_FOREGROUND_SET: Arc<Mutex<bool>> = Arc::new(Mutex::new(false));
}

pub enum WindowManagerMessage {
    CloseWindow,
    EnumerateWindows,
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

    let mut msg: MSG = Default::default();

    loop {
        unsafe {
            if PeekMessageW(&mut msg, null_mut(), 0, 0, PM_REMOVE) != 0 {
                if msg.message == WM_HOTKEY {
                    if let Ok(mut gaurd) = LEADER_PRESSED.lock() {
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
        //Check if a new foreground has been set without using the hotkeys
        match NEW_FOREGROUND_SET.lock() {
            Ok(gaurd) => {
                if *gaurd {
                    window_manager.set_windows();
                    unsafe {
                        window_manager.current = Window::new(GetForegroundWindow(), 0);
                    }
                }
            }
            Err(e) => println!("Failed to lock mutex: {}", e),
        }
    }
    unregister_leader();
    Ok(())
}

