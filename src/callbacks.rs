use crate::window::Window;
use crate::window_manager::WindowManager;
use crate::{LEADER_PRESSED, NEW_FOREGROUND_SET};
use std::sync::atomic::Ordering;
use winapi::shared::minwindef::{DWORD, LPARAM};
use winapi::shared::ntdef::LONG;
use winapi::shared::windef::{HWINEVENTHOOK, HWND};
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{
    GetWindowTextW, IsWindowVisible, EVENT_SYSTEM_FOREGROUND, SW_HIDE, SW_SHOWMINIMIZED,
};
//Checks if leader is pressed and signals window manager to re enumerate windows
pub unsafe extern "system" fn win_event_proc(
    _: HWINEVENTHOOK,
    event: u32,
    _hwnd: HWND,
    _: LONG,
    _: LONG,
    _: DWORD,
    _: DWORD,
) {
    let leader_pressed = LEADER_PRESSED.load(Ordering::Acquire);
    if event == EVENT_SYSTEM_FOREGROUND && !leader_pressed {
        println!("leader in callback {}", leader_pressed);
        NEW_FOREGROUND_SET.store(true, Ordering::Relaxed);
    }
}

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let mut buffer: [WCHAR; 1024] = [0; 1024];
    let window_manager = &mut *(lparam as *mut WindowManager);
    let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

    if length > 0 && IsWindowVisible(hwnd) != 0 && hwnd != window_manager.current.hwnd {
        window_manager.count += 1;
        let window = Window::new(hwnd, window_manager.count);
        if window.placement.showCmd != SW_HIDE as u32
            && window.placement.showCmd != SW_SHOWMINIMIZED as u32
        {
            if let Some(title) = window.get_title() {
                if title != "Windows Input Experience" {
                    window_manager.set_window(window);
                }
            }
        }
    }
    1
}

