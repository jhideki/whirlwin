mod win_api {
    pub mod window_manager;
}
mod callbacks;
mod handle;
use callbacks::{keyboard_callback, shell_hook_callback};
use std::{io::Write, ptr::null_mut};

use handle::Handle;
use std::io;
use win_api::window_manager;
use winapi::um::winuser;

fn main() {
    let mut windows: Vec<Handle> = Vec::new();
    unsafe { windows = window_manager::get_windows() }
    for window in &windows {
        window.print_title();
    }

    match window_manager::switch_to_window(windows[1].hwnd) {
        Ok(()) => windows[1].print_title(),
        Err(err) => eprintln!("Error: {}", err),
    }

    unsafe {
        let keyboard_hook = winuser::SetWindowsHookExW(
            winuser::WH_KEYBOARD,
            Some(keyboard_callback),
            null_mut(),
            0,
        );

        if keyboard_hook.is_null() {
            println!("error in keyboard hook");
        }
        //Listen for creations of windows
        let shell_hook =
            winuser::SetWindowsHookExW(winuser::WH_SHELL, Some(shell_hook_callback), null_mut(), 0);

        if shell_hook.is_null() {
            println!("error with shell hook");
        }

        loop {
            println!("listening");
            io::stdout().flush().expect("failed to flush stoud");
            let mut input = String::new();
            io::stdin().read_line(&mut input);
            if input.trim() == "exit" {
                println!("exiting");
                break;
            }
        }

        //Main event loop
        winuser::UnhookWindowsHookEx(keyboard_hook);
        winuser::UnhookWindowsHookEx(shell_hook);
    }
}
