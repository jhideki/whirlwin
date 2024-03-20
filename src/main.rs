mod callbacks;
mod keybinds;
mod window;
mod window_manager;

use callbacks::win_event_proc;
use keybinds::{handle_hotkey, register_leader, unregister_leader};
use window_manager::{WindowManager, WindowManagerMessage};

use std::io::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, TranslateMessage, EVENT_SYSTEM_FOREGROUND, MSG,
    WINEVENT_OUTOFCONTEXT, WM_HOTKEY, WM_USER,
};

static LEADER_PRESSED: AtomicBool = AtomicBool::new(false);
const NEW_FOREGROUND_SET: u32 = WM_USER + 1;

fn spawn_hook(sender: Arc<Sender<WindowManagerMessage>>) {
    unsafe {
        SetWinEventHook(
            EVENT_SYSTEM_FOREGROUND,
            EVENT_SYSTEM_FOREGROUND,
            None,
            Some(win_event_proc),
            0,
            0,
            WINEVENT_OUTOFCONTEXT,
        );
    }

    let mut msg: MSG = MSG::default();
    unsafe {
        loop {
            if GetMessageW(&mut msg, None, 0, 0).into() {
                if msg.message == NEW_FOREGROUND_SET {
                    if let Err(err) = sender.send(WindowManagerMessage::SetCurrent) {
                        println!("{}", err);
                    }
                    if let Err(err) = sender.send(WindowManagerMessage::SetWindows) {
                        println!("{}", err);
                    }
                }
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn key_listener(sender: Arc<Sender<WindowManagerMessage>>) {
    unsafe {
        let mut msg: MSG = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            if msg.message == WM_HOTKEY {
                let leader_pressed = LEADER_PRESSED.load(Ordering::Relaxed);
                let wparam = msg.wParam.0 as i32;
                match handle_hotkey(wparam, &sender, leader_pressed) {
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

fn main() -> Result<(), Error> {
    let (sender, receiver) = channel();
    let sender_arc = Arc::new(sender);

    let mut window_manager = WindowManager::new(receiver);
    window_manager.set_windows();
    let window_manger_listener = thread::spawn(move || window_manager.start());
    let callback_listener = {
        let sender = Arc::clone(&sender_arc);
        thread::spawn(move || spawn_hook(sender))
    };

    unregister_leader();
    match register_leader() {
        Ok(()) => println!("Leader Registered"),
        Err(e) => println!("Failed to registrer leader: {}", e),
    }

    key_listener(Arc::clone(&sender_arc));

    window_manger_listener.join().unwrap();
    println!("callback listenern joined");
    callback_listener.join().unwrap();
    unregister_leader();
    Ok(())
}

