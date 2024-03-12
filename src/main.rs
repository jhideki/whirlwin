mod callbacks;
mod keybinds;
mod window;
mod window_manager;

use callbacks::win_event_proc;
use keybinds::{handle_hotkey, register_leader, unregister_leader};
use std::io::Error;
use std::ptr::{self, null_mut};
use std::sync::atomic::{AtomicBool, Ordering};
use winapi::um::winuser::{
    GetForegroundWindow, PeekMessageW, SetWinEventHook, EVENT_SYSTEM_FOREGROUND, MSG, PM_REMOVE,
    WINEVENT_OUTOFCONTEXT, WM_HOTKEY,
};
use window::Window;
use window_manager::WindowManager;

//I couldn't think of a better way to signal the window manager from the event hook
static LEADER_PRESSED: AtomicBool = AtomicBool::new(false);
static NEW_FOREGROUND_SET: AtomicBool = AtomicBool::new(false);

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
                    let leader_pressed = LEADER_PRESSED.load(Ordering::Relaxed);
                    match handle_hotkey(msg.wParam as i32, &mut window_manager, leader_pressed) {
                        Ok(leader) => {
                            println!("leader after hotkey {}", leader);
                            LEADER_PRESSED.store(leader, Ordering::Relaxed);
                        }
                        Err(e) => {
                            println!("Error {}", e);
                            break;
                        }
                    }
                }
            }
        }
        if NEW_FOREGROUND_SET.load(Ordering::Relaxed) {
            //Check if a new foreground has been set without using the hotkeys
            window_manager.clear_windows();
            window_manager.set_windows();
            unsafe {
                window_manager.current = Window::new(GetForegroundWindow(), 0);
            }
            println!("new foreground set through clicking");
            NEW_FOREGROUND_SET.store(false, Ordering::Relaxed);
        }
    }
    unregister_leader();
    Ok(())
}

