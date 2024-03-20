use crate::window::Window;
use crate::window_manager::WindowManager;
use crate::{LEADER_PRESSED, NEW_FOREGROUND_SET};
use std::sync::atomic::Ordering;
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::UI::Accessibility::HWINEVENTHOOK;
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowTextW, IsWindowVisible, PostMessageW, EVENT_SYSTEM_FOREGROUND,
};

//Checks if leader is pressed and signals window manager to re enumerate windows
pub unsafe extern "system" fn win_event_proc(
    _: HWINEVENTHOOK,
    event: u32,
    _hwnd: HWND,
    _: i32,
    _: i32,
    _: u32,
    _: u32,
) {
    println!("callback called");
    let leader_pressed = LEADER_PRESSED.load(Ordering::Acquire);
    if event == EVENT_SYSTEM_FOREGROUND && !leader_pressed {
        let _ = PostMessageW(None, NEW_FOREGROUND_SET, None, None);
        println!("leader in callback {}", leader_pressed);
    }

    println!("callback finisehd");
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> BOOL {
    let mut buffer: [u16; 1024] = [0; 1024];
    let window_manager = &mut *(lparam.0 as *mut WindowManager);
    let length = GetWindowTextW(hwnd, &mut buffer);

    if length > 0 && IsWindowVisible(hwnd).as_bool() && hwnd != window_manager.current.hwnd {
        window_manager.count += 1;
        let window = Window::new(hwnd, window_manager.count);
        //SW_HIDE and SW_SHOWMINIMIZED
        if window.placement.showCmd != 0 && window.placement.showCmd != 2 {
            if let Some(title) = window.get_title() {
                if title != "Windows Input Experience" {
                    window_manager.set_window(window);
                }
            }
        }
    }
    true.into()
}

