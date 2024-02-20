use crate::window::Window;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{
    EnumWindows, GetForegroundWindow, GetWindowTextW, IsWindowVisible, SetForegroundWindow,
};
pub struct WindowManager {
    pub current: Window,
    pub left: Option<Window>,
    pub right: Option<Window>,
    pub below: Option<Window>,
    pub above: Option<Window>,
    pub behind: Option<Window>,
}
impl WindowManager {
    pub fn new() -> WindowManager {
        let current = unsafe { Window::new(GetForegroundWindow()) };
        println!("Current window:");
        current.print_title();
        WindowManager {
            current,
            left: None,
            right: None,
            below: None,
            above: None,
            behind: None,
        }
    }

    fn set_window(&mut self, window: Window) {
        if window.rect.left < self.current.rect.left {
            println!("left window:");
            window.print_title();
            self.left = Some(window);
        }
    }

    pub fn set_windows(&mut self) {
        unsafe {
            EnumWindows(Some(enum_windows_proc), self as *mut _ as LPARAM);
        }
    }

    pub fn switch_to_window(&self) -> Result<(), String> {
        match &self.left {
            Some(window) => unsafe {
                if SetForegroundWindow(window.hwnd) == 0 {
                    Err("Failed to switch windows".to_string())
                } else {
                    Ok(())
                }
            },
            None => Err("Left window does not exist".to_string()),
        }
    }
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let mut buffer: [WCHAR; 1024] = [0; 1024];
    let window_manager = &mut *(lparam as *mut WindowManager);
    let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

    if length > 0 && IsWindowVisible(hwnd) != 0 {
        let window = Window::new(hwnd);
        window_manager.set_window(window);
    }
    1
}

