mod callbacks;
mod keybinds;
mod user_config;
mod window;
mod window_manager;

use callbacks::win_event_proc;
use keybinds::{handle_hotkey, register_leader, unregister_leader};
use window_manager::{WindowManager, WindowManagerMessage};

use std::error::Error;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{channel, Sender};
use std::sync::Arc;
use std::thread;

use crossbeam_channel;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Accessibility::SetWinEventHook;
use windows::Win32::UI::WindowsAndMessaging::{
    DispatchMessageW, GetMessageW, PostThreadMessageW, TranslateMessage, EVENT_SYSTEM_FOREGROUND,
    MSG, WINEVENT_OUTOFCONTEXT, WM_HOTKEY, WM_USER,
};

static HOTKEY_PRESSED: AtomicBool = AtomicBool::new(false);
const NEW_FOREGROUND_SET: u32 = WM_USER + 1;
const EXIT_PROGRAM: u32 = WM_USER + 2;

fn spawn_hook(
    sender: Arc<Sender<WindowManagerMessage>>,
    thread_id_sender: crossbeam_channel::Sender<u32>,
) {
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
    let thread_id = unsafe { GetCurrentThreadId() };
    if let Err(e) = thread_id_sender.send(thread_id) {
        println!("Error sending thread id {}", e);
    }

    let mut msg: MSG = MSG::default();
    loop {
        unsafe {
            if GetMessageW(&mut msg, None, 0, 0).into() {
                if msg.message == NEW_FOREGROUND_SET {
                    if let Err(err) = sender.send(WindowManagerMessage::ClearWindows) {
                        println!("{}", err);
                    }
                    if let Err(err) = sender.send(WindowManagerMessage::SetCurrent) {
                        println!("{}", err);
                    }
                    if let Err(err) = sender.send(WindowManagerMessage::SetWindows) {
                        println!("{}", err);
                    }
                } else if msg.message == EXIT_PROGRAM {
                    break;
                }
            }
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn key_listener(
    sender: Arc<Sender<WindowManagerMessage>>,
    callback_thread_id: u32,
) -> Result<(), Box<dyn Error>> {
    let config = user_config::read_config()?;

    let mut leader_pressed = false;
    unsafe {
        let mut msg: MSG = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            if msg.message == WM_HOTKEY {
                HOTKEY_PRESSED.store(true, Ordering::Relaxed);
                let wparam = msg.wParam.0 as i32;
                match handle_hotkey(wparam, &sender, leader_pressed, &config) {
                    Ok(leader) => {
                        leader_pressed = leader;
                    }
                    Err(e) => {
                        if let Err(e) =
                            PostThreadMessageW(callback_thread_id, EXIT_PROGRAM, None, None)
                        {
                            println!("error sending thread message {}", e);
                        }
                        println!("Error {}", e);
                        break;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let (sender, receiver) = channel();
    let sender_arc = Arc::new(sender);

    let mut window_manager = WindowManager::new(receiver);
    window_manager.set_windows();
    let window_manger_listener = thread::spawn(move || window_manager.start());

    unregister_leader();
    match register_leader() {
        Ok(()) => println!("Leader Registered"),
        Err(e) => println!("Failed to registrer leader: {}", e),
    }

    //Channel used for retreiving thread id of callback listener
    let (callback_sender, callback_receiver) = crossbeam_channel::unbounded();
    let callback_listener = {
        let thread_id_sender = callback_sender.clone();
        let sender = Arc::clone(&sender_arc);
        thread::spawn(move || spawn_hook(sender, thread_id_sender))
    };

    let callback_thread_id = callback_receiver.recv().unwrap();
    key_listener(Arc::clone(&sender_arc), callback_thread_id);

    window_manger_listener.join().unwrap();
    callback_listener.join().unwrap();
    unregister_leader();
    Ok(())
}
