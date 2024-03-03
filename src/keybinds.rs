use crate::switch_to_direction;
use crate::window_manager::WindowManager;
use std::io::Error;
use winapi::um::winuser::{RegisterHotKey, SetForegroundWindow, UnregisterHotKey};

use std::ptr::null_mut;

use winapi::um::winuser::{MOD_SHIFT, VK_ESCAPE, VK_SPACE};

//Hotkey indentifies
const EXIT: i32 = 1;
const SWITCH_LEFT: i32 = 2;
const SWITCH_RIGHT: i32 = 3;
const SWITCH_ABOVE: i32 = 4;
const SWITCH_BELOW: i32 = 5;
const SWITCH_BEHIND: i32 = 6;
const LEADER: i32 = 7;
const CLOSE_WINDOW: i32 = 8;

//Keycode
const KEY_H: i32 = 0x48;
const KEY_L: i32 = 0x4C;
const KEY_J: i32 = 0x4A;
const KEY_K: i32 = 0x4B;
const KEY_N: i32 = 0x4E;
const KEY_D: i32 = 0x44;

pub fn handle_hotkey(
    wparam: i32,
    window_manager: &mut WindowManager,
    mut leader_pressed: bool,
) -> Result<bool, String> {
    if !leader_pressed && wparam == LEADER {
        leader_pressed = true;
        match register_hotkeys() {
            Ok(_) => return Ok(leader_pressed),
            Err(e) => return Err(format!("Error: {}", e)),
        }
    } else if leader_pressed {
        match wparam {
            EXIT => return Err("Exiting the program".to_string()),
            SWITCH_LEFT => unsafe { switch_to_direction!(window_manager, left) },
            SWITCH_RIGHT => unsafe { switch_to_direction!(window_manager, right) },
            SWITCH_ABOVE => unsafe { switch_to_direction!(window_manager, above) },
            SWITCH_BELOW => unsafe { switch_to_direction!(window_manager, below) },
            SWITCH_BEHIND => unsafe { switch_to_direction!(window_manager, behind) },
            CLOSE_WINDOW => window_manager.close_window(),
            _ => println!("idk bru"),
        }
        leader_pressed = false;
        unregister_hotkeys();
    }
    Ok(leader_pressed)
}
pub fn register_leader() -> Result<(), Error> {
    unsafe {
        if RegisterHotKey(null_mut(), LEADER, MOD_SHIFT as u32, VK_SPACE as u32) == 0 {
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn unregister_leader() {
    unsafe {
        UnregisterHotKey(null_mut(), LEADER);
    }
}
fn register_hotkeys() -> Result<(), Error> {
    unsafe {
        // Exit program
        if RegisterHotKey(null_mut(), EXIT, 0, VK_ESCAPE as u32) == 0 {
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_LEFT, 0, KEY_H as u32) == 0 {
            println!("failed to register H");
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_RIGHT, 0, KEY_L as u32) == 0 {
            println!("failed to register L");
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_ABOVE, 0, KEY_K as u32) == 0 {
            println!("failed to register K");
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_BELOW, 0, KEY_J as u32) == 0 {
            println!("failed to register J");
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_BEHIND, 0, KEY_N as u32) == 0 {
            println!("failed to register N");
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), CLOSE_WINDOW, 0, KEY_D as u32) == 0 {
            println!("failed to register D");
            return Err(Error::last_os_error());
        }
    }
    Ok(())
}

pub fn unregister_hotkeys() {
    unsafe {
        UnregisterHotKey(null_mut(), EXIT);
        UnregisterHotKey(null_mut(), SWITCH_LEFT);
        UnregisterHotKey(null_mut(), SWITCH_RIGHT);
        UnregisterHotKey(null_mut(), SWITCH_ABOVE);
        UnregisterHotKey(null_mut(), SWITCH_BELOW);
        UnregisterHotKey(null_mut(), SWITCH_BEHIND);
        UnregisterHotKey(null_mut(), CLOSE_WINDOW);
    }
}

