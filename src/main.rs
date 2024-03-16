mod callbacks;
mod keybinds;
mod window;
mod window_manager;

use async_std::channel::{Receiver, Send, Sender};
use async_std::{channel, task};
use callbacks::win_event_proc;
use keybinds::{handle_hotkey, register_leader, unregister_leader};
use lazy_static::lazy_static;
use std::io::Error;
use std::ptr::{self, null_mut};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::GetMessageW;
use windows::Win32::UI::WindowsAndMessaging::MSG;

use winapi::um::winuser::{
    GetForegroundWindow, SetWinEventHook, EVENT_SYSTEM_FOREGROUND, PM_REMOVE,
    WINEVENT_OUTOFCONTEXT, WM_HOTKEY, WM_USER,
};
use window::Window;
use window_manager::{WindowManager, WindowManagerMessage};

//I couldn't think of a better way to signal the window manager from the event hook
static LEADER_PRESSED: AtomicBool = AtomicBool::new(false);
static NEW_FOREGROUND_SET: AtomicBool = AtomicBool::new(false);
lazy_static! {
    static ref CALLBACK_CONDVAR: Condvar = Condvar::new();
    static ref CALLBACK_CALLED: Mutex<bool> = Mutex::new(false);
}

fn handle_callback(window_manager: Arc<Mutex<WindowManager>>) {
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
    loop {
        let mut gaurd = CALLBACK_CALLED.lock().unwrap();
        while !*gaurd {
            gaurd = CALLBACK_CONDVAR.wait(gaurd).unwrap();
        }
        if NEW_FOREGROUND_SET.load(Ordering::Relaxed) {
            let mut gaurd = window_manager.lock().unwrap();
            //Check if a new foreground has been set without using the hotkeys
            gaurd.clear_windows();
            gaurd.set_windows();
            unsafe {
                gaurd.current = Window::new(GetForegroundWindow(), 0);
            }
            println!("new foreground set through clicking");
            NEW_FOREGROUND_SET.store(false, Ordering::Relaxed);
        }
    }
}

async fn key_listener(sender: Arc<Sender<WindowManagerMessage>>) {
    loop {
        unsafe {
            let mut msg: MSG = MSG::default();
            if !GetMessageW(&mut msg, HWND(0), 0, 0).as_bool() {
                if msg.message == WM_HOTKEY {
                    let leader_pressed = LEADER_PRESSED.load(Ordering::Relaxed);
                    let wparam = msg.wParam.0 as i32;
                    match handle_hotkey(wparam, &sender, leader_pressed).await {
                        Ok(leader) => {
                            println!("leader after hotkey {}", leader);
                            LEADER_PRESSED.store(leader, Ordering::Relaxed);
                            println!("leader pressed updated");
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
}

fn main() -> Result<(), Error> {
    let (sender, receiver) = channel::unbounded();
    let sender_arc = Arc::new(sender);

    let mut window_manager = WindowManager::new(receiver);
    window_manager.set_windows();
    let key_listener = task::spawn(key_listener(sender_arc.clone()));
    let window_listner = task::block_on(window_manager.start());

    unregister_leader();
    match register_leader() {
        Ok(()) => println!("Leader Registered"),
        Err(e) => println!("Failed to registrer leader: {}", e),
    }

    unregister_leader();
    Ok(())
}

