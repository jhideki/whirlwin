use crate::window_manager::{Direction, WindowManagerMessage};
use async_std::channel::Sender;
use std::io::Error;
use std::sync::{Arc, Mutex};
use winapi::um::winuser::{
    RegisterHotKey, SetForegroundWindow, UnregisterHotKey, VK_CAPITAL, VK_CONTROL, VK_RETURN,
};

use std::ptr::null_mut;

use winapi::um::winuser::{MOD_SHIFT, VK_ESCAPE, VK_SPACE};
use windows::Win32::Foundation::WPARAM;

//Hotkey indentifies
const EXIT: i32 = 1;
const SWITCH_LEFT: i32 = 2;
const SWITCH_RIGHT: i32 = 3;
const SWITCH_ABOVE: i32 = 4;
const SWITCH_BELOW: i32 = 5;
const SWITCH_BEHIND: i32 = 6;
const LEADER: i32 = 7;
const CLOSE_WINDOW: i32 = 8;
const SWITCH_PREVIOUS: i32 = 9;

//Keycode
const KEY_H: i32 = 0x48;
const KEY_L: i32 = 0x4C;
const KEY_J: i32 = 0x4A;
const KEY_K: i32 = 0x4B;
const KEY_N: i32 = 0x4E;
const KEY_D: i32 = 0x44;
const KEY_P: i32 = 0x50;

pub async fn handle_hotkey(
    wparam: i32,
    sender: &Arc<Sender<WindowManagerMessage>>,
    leader_pressed: bool,
) -> Result<bool, String> {
    if !leader_pressed && wparam == LEADER {
        match register_hotkeys() {
            Ok(_) => return Ok(true),
            Err(e) => return Err(format!("Error: {}", e)),
        };
    }
    if leader_pressed {
        match wparam {
            EXIT => return Err("User hit ESC.".to_string()),
            SWITCH_LEFT => {
                sender
                    .send(WindowManagerMessage::SwitchToDirection(Direction::Left))
                    .await;
            }
            SWITCH_RIGHT => {
                sender
                    .send(WindowManagerMessage::SwitchToDirection(Direction::Right))
                    .await;
            }
            SWITCH_ABOVE => {
                sender
                    .send(WindowManagerMessage::SwitchToDirection(Direction::Above))
                    .await;
            }
            SWITCH_BELOW => {
                sender
                    .send(WindowManagerMessage::SwitchToDirection(Direction::Below))
                    .await;
            }
            SWITCH_BEHIND => {
                sender.send(WindowManagerMessage::SwitchToNext).await;
            }
            CLOSE_WINDOW => {
                sender.send(WindowManagerMessage::CloseWindow).await;
            }
            SWITCH_PREVIOUS => {
                sender.send(WindowManagerMessage::SwitchToPrevious).await;
            }
            _ => {
                println!("idk bru");
            }
        }

        unregister_hotkeys();
    }
    Ok(false)
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

        if RegisterHotKey(null_mut(), SWITCH_PREVIOUS, 0, KEY_P as u32) == 0 {
            println!("failed to register P");
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
        UnregisterHotKey(null_mut(), SWITCH_PREVIOUS);
    }
}

