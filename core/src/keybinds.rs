use crate::window_manager::{Direction, WindowManagerMessage};

use std::io::Error;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS,
};

//Hotkey indentifies
const EXIT: i32 = 1;
const SWITCH_LEFT: i32 = 2;
const SWITCH_RIGHT: i32 = 3;
const SWITCH_ABOVE: i32 = 4;
const SWITCH_BELOW: i32 = 5;
const SWITCH_NEXT: i32 = 6;
const LEADER: i32 = 7;
const CLOSE_WINDOW: i32 = 8;
const SWITCH_PREVIOUS: i32 = 9;

//Keycode
const KEY_H: u32 = 0x48;
const KEY_L: u32 = 0x4C;
const KEY_J: u32 = 0x4A;
const KEY_K: u32 = 0x4B;
const KEY_N: u32 = 0x4E;
const KEY_D: u32 = 0x44;
const KEY_P: u32 = 0x50;
const ESC: u32 = 0x1B;
const SHIFT: u32 = 0x10;
const SPACE: u32 = 0x20;
const ENTER: u32 = 0x0D;
const CAPS: u32 = 0x14;

pub fn handle_hotkey(
    wparam: i32,
    sender: &Arc<Sender<WindowManagerMessage>>,
    leader_pressed: bool,
) -> Result<bool, String> {
    if !leader_pressed && wparam == LEADER {
        println!("leader pressed");
        match register_hotkeys() {
            Ok(_) => return Ok(true),
            Err(e) => return Err(format!("Error: {}", e)),
        };
    }
    if leader_pressed {
        match wparam {
            EXIT => {
                if let Err(err) = sender.send(WindowManagerMessage::EndListener) {
                    return Err(format!("Failed to send message: {}", err));
                }
                return Err("User hit ESC.".to_string());
            }
            SWITCH_LEFT => {
                if let Err(err) =
                    sender.send(WindowManagerMessage::SwitchToDirection(Direction::Left))
                {
                    println!("Failed to send message: {}", err);
                }
            }
            SWITCH_RIGHT => {
                if let Err(err) =
                    sender.send(WindowManagerMessage::SwitchToDirection(Direction::Right))
                {
                    println!("Failed to send message: {}", err);
                }
            }
            SWITCH_ABOVE => {
                if let Err(err) =
                    sender.send(WindowManagerMessage::SwitchToDirection(Direction::Above))
                {
                    println!("Failed to send message: {}", err);
                }
            }
            SWITCH_BELOW => {
                if let Err(err) =
                    sender.send(WindowManagerMessage::SwitchToDirection(Direction::Below))
                {
                    println!("Failed to send message: {}", err);
                }
            }
            SWITCH_NEXT => {
                if let Err(err) = sender.send(WindowManagerMessage::SwitchToNext) {
                    println!("Failed to send message: {}", err);
                }
            }
            CLOSE_WINDOW => {
                if let Err(err) = sender.send(WindowManagerMessage::CloseWindow) {
                    println!("Failed to send message: {}", err);
                }
            }
            SWITCH_PREVIOUS => {
                if let Err(err) = sender.send(WindowManagerMessage::SwitchToPrevious) {
                    println!("Failed to send message: {}", err);
                }
            }
            _ => {
                println!("idk bru");
            }
        }
    }

    unregister_hotkeys();
    Ok(false)
}
pub fn register_leader() -> Result<(), Error> {
    unsafe {
        if let Err(e) = RegisterHotKey(None, LEADER, HOT_KEY_MODIFIERS(4 | 0x4000), ENTER) {
            println!("{}", e);
        }
    }
    Ok(())
}

pub fn unregister_leader() {
    unsafe {
        let _ = UnregisterHotKey(None, LEADER);
    }
}
fn register_hotkeys() -> Result<(), Error> {
    unsafe {
        if let Err(_) = RegisterHotKey(None, EXIT, HOT_KEY_MODIFIERS(0), ESC) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_LEFT, HOT_KEY_MODIFIERS(0), KEY_H) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_RIGHT, HOT_KEY_MODIFIERS(0), KEY_L) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_BELOW, HOT_KEY_MODIFIERS(0), KEY_K) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_ABOVE, HOT_KEY_MODIFIERS(0), KEY_J) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_NEXT, HOT_KEY_MODIFIERS(0), KEY_N) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, CLOSE_WINDOW, HOT_KEY_MODIFIERS(0), KEY_D) {
            println!("failed to register ESC");
        }

        if let Err(_) = RegisterHotKey(None, SWITCH_PREVIOUS, HOT_KEY_MODIFIERS(0), KEY_P) {
            println!("failed to register ESC");
        }
    }
    Ok(())
}

pub fn unregister_hotkeys() {
    unsafe {
        let _ = UnregisterHotKey(None, EXIT);
        let _ = UnregisterHotKey(None, SWITCH_LEFT);
        let _ = UnregisterHotKey(None, SWITCH_RIGHT);
        let _ = UnregisterHotKey(None, SWITCH_ABOVE);
        let _ = UnregisterHotKey(None, SWITCH_BELOW);
        let _ = UnregisterHotKey(None, SWITCH_NEXT);
        let _ = UnregisterHotKey(None, CLOSE_WINDOW);
        let _ = UnregisterHotKey(None, SWITCH_PREVIOUS);
    }
}

