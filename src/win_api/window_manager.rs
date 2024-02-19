use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{EnumWindows, GetWindowTextW, IsWindowVisible, SetForegroundWindow};

use crate::handle::Handle;

pub unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let mut buffer: [WCHAR; 1024] = [0; 1024];
    let window_titles = &mut *(lparam as *mut Vec<Handle>);
    let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

    if length > 0 && IsWindowVisible(hwnd) != 0 {
        let window_title = String::from_utf16_lossy(&buffer[..length as usize]);
        window_titles.push(Handle { hwnd: hwnd });
    }
    1
}

pub fn switch_to_window(hwnd: HWND) -> Result<(), String> {
    unsafe {
        if SetForegroundWindow(hwnd) == 0 {
            Err("Failed to switch windows".to_string())
        } else {
            Ok(())
        }
    }
}

pub unsafe fn get_windows() -> Vec<Handle> {
    let mut window_titles: Vec<Handle> = Vec::new();
    EnumWindows(
        Some(enum_windows_proc),
        &mut window_titles as *mut _ as LPARAM,
    );
    window_titles
}

