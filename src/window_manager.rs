use std::ptr::null_mut;

use crate::window::Window;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{
    EnumWindows, GetForegroundWindow, GetWindow, GetWindowTextW, IsWindowVisible,
    SetForegroundWindow, SW_HIDE,
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
        window.print_title();
        println!("flags {}", window.placement.flags);
        println!("dwStyle {}", window.info.dwStyle);
        println!("creator version{}", window.info.wCreatorVersion);
        println!("atom {}", window.info.atomWindowType);
        println!("visibility {}", unsafe { IsWindowVisible(window.hwnd) });
        println!("border width {}", window.info.cxWindowBorders);
        println!("cmd {}", window.placement.showCmd);
        println!("left {}", window.rect.left);
        println!("right {}", window.rect.right);
        println!("bottom {}", window.rect.bottom);
        println!("top {}", window.rect.top);
        println!("");
        if self.left.is_none() && window.rect.right <= self.current.rect.left {
            self.left = Some(window);
        } else if self.right.is_none() && window.rect.left >= self.current.rect.right {
            self.right = Some(window);
        } else if self.above.is_none() && window.rect.bottom >= self.current.rect.top {
            self.above = Some(window);
        } else if self.below.is_none() && window.rect.top <= self.current.rect.bottom {
            self.below = Some(window);
        } else if self.behind.is_none() && window.rect.left >= self.current.rect.left {
            self.behind = Some(window)
        }
    }
    pub fn print_windows(&mut self) {
        print!("left");
        match &self.left {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
        }

        print!("right");
        match &self.right {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
        }

        print!("above");
        match &self.above {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
        }

        print!("below");
        match &self.below {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
        }

        print!("behind");
        match &self.behind {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
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
        if window.placement.showCmd != SW_HIDE as u32 {
            window_manager.set_window(window);
        }
    }
    1
}

