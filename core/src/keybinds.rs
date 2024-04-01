use crate::user_config::Config;
use crate::window_manager::{Direction, WindowManagerMessage};

use std::io::Error;
use std::path::PathBuf;
use std::process::Command;
use std::sync::mpsc::Sender;
use std::sync::Arc;

use windows::Win32::UI::Input::KeyboardAndMouse::{
    RegisterHotKey, UnregisterHotKey, HOT_KEY_MODIFIERS,
};

//Hotkey indentifies

const LEADER: i32 = 0;
const EXIT: i32 = 1;
const SWITCH_LEFT: i32 = 2;
const SWITCH_RIGHT: i32 = 3;
const SWITCH_ABOVE: i32 = 4;
const SWITCH_BELOW: i32 = 5;
const SWITCH_NEXT: i32 = 6;
const CLOSE_WINDOW: i32 = 8;
const SWITCH_PREVIOUS: i32 = 9;
const SHORTCUT_1: i32 = 10;
const SHORTCUT_2: i32 = 11;
const SHORTCUT_3: i32 = 12;
const SHORTCUT_4: i32 = 13;
const SHORTCUT_5: i32 = 14;
const SHORTCUT_6: i32 = 15;
const SHORTCUT_7: i32 = 16;
const SHORTCUT_8: i32 = 17;
const SHORTCUT_9: i32 = 18;
const SHORTCUT_10: i32 = 19;

//Keycodes
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
const KEY_0: u32 = 0x30;
const KEY_1: u32 = 0x31;
const KEY_2: u32 = 0x32;
const KEY_3: u32 = 0x33;
const KEY_4: u32 = 0x34;
const KEY_5: u32 = 0x35;
const KEY_6: u32 = 0x36;
const KEY_7: u32 = 0x37;
const KEY_8: u32 = 0x38;
const KEY_9: u32 = 0x39;

pub fn handle_hotkey(
    wparam: i32,
    sender: &Arc<Sender<WindowManagerMessage>>,
    leader_pressed: bool,
    config: &Config,
) -> Result<bool, String> {
    if !leader_pressed && wparam == LEADER {
        println!("leader pressed");
        match register_hotkeys(config) {
            Ok(_) => return Ok(true),
            Err(e) => return Err(format!("Error: {}", e)),
        };
    }
    if leader_pressed {
        println!("leader pressed");
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
            SHORTCUT_1 => {
                let _ = Command::new(PathBuf::from(config.programs[0].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_2 => {
                let _ = Command::new(PathBuf::from(config.programs[1].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_3 => {
                let _ = Command::new(PathBuf::from(config.programs[2].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_4 => {
                let _ = Command::new(PathBuf::from(config.programs[3].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_5 => {
                let _ = Command::new(PathBuf::from(config.programs[4].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_6 => {
                let _ = Command::new(PathBuf::from(config.programs[5].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_7 => {
                let _ = Command::new(PathBuf::from(config.programs[6].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_8 => {
                let _ = Command::new(PathBuf::from(config.programs[7].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_9 => {
                let _ = Command::new(PathBuf::from(config.programs[8].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
            }
            SHORTCUT_10 => {
                let _ = Command::new(PathBuf::from(config.programs[9].clone()))
                    .status()
                    .expect("Failed to oppen program 1");
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
fn register_hotkeys(config: &Config) -> Result<(), Error> {
    unsafe {
        for i in 0..config.programs.len() {
            if config.programs[i] != "" {
                if let Err(e) =
                    RegisterHotKey(None, i as i32 + 10, HOT_KEY_MODIFIERS(0), i as u32 + 0x31)
                {
                    println!("failed to register key {}. Error: {}", i as u32 + 0x30, e);
                } else {
                    println!("registered key {}", i + 0x30);
                }
            }
        }
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
        let _ = UnregisterHotKey(None, SWITCH_PREVIOUS);
        // Unregister shortcuts
        for i in 10..20 {
            let _ = UnregisterHotKey(None, i as i32);
        }
    }
}
