use std::ptr::null_mut;

use crate::window::Window;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{
    EnumWindows, GetClassNameW, GetForegroundWindow, GetWindow, GetWindowTextW, IsWindowVisible,
    SetForegroundWindow, GW_HWNDNEXT, SW_HIDE, SW_MINIMIZE, SW_SHOWMINIMIZED,
};

pub struct WindowManager {
    pub current: Window,
    pub left: Option<Window>,
    pub right: Option<Window>,
    pub below: Option<Window>,
    pub above: Option<Window>,
    pub behind: Option<Window>,
    pub count: i32, // corresponds to order in window struct
}
impl WindowManager {
    pub fn new() -> WindowManager {
        let current = unsafe { Window::new(GetForegroundWindow(), 0) };
        Self::print_window_info(&current);
        WindowManager {
            current,
            left: None,
            right: None,
            below: None,
            above: None,
            behind: None,
            count: 0,
        }
    }

    fn set_window(&mut self, window: Window) {
        window.print_title();
        let mut class_name: [u16; 256] = [0; 256];

        unsafe {
            let len = GetClassNameW(window.hwnd, class_name.as_mut_ptr(), 256);
            let class = String::from_utf16_lossy(&class_name[..len as usize]);
            println!("class name {}", class);
        }

        Self::print_window_info(&window);
        if self.left.is_none() && window.rect.right <= self.current.rect.left {
            self.left = Some(window);
        } else if self.right.is_none() && window.rect.left >= self.current.rect.right {
            self.right = Some(window);
        } else if self.above.is_none() && window.rect.bottom <= self.current.rect.top {
            self.above = Some(window);
        } else if self.below.is_none() && window.rect.top >= self.current.rect.bottom {
            self.below = Some(window);
        } else if self.behind.is_none() && (window.rect.left >= self.current.rect.left)
            || window.rect.right <= self.current.rect.right
        {
            self.behind = Some(window)
        }
    }
    fn print_window_info(window: &Window) {
        println!("order: {}", window.order);
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

    pub fn get_all_windows(&mut self) {
        let mut hwnd = unsafe { GetWindow(self.current.hwnd, GW_HWNDNEXT) };
        let mut order = 0;
        while !hwnd.is_null() {
            let window = Window::new(hwnd, order);
            if unsafe { IsWindowVisible(hwnd) } == 1 && window.placement.showCmd != 2 {
                self.set_window(window);
            }
            hwnd = unsafe { GetWindow(hwnd, GW_HWNDNEXT) };
            order += 1;
        }
    }

    pub fn switch_to_left(&mut self) -> Result<(), String> {
        match &self.left {
            Some(window) => unsafe {
                if SetForegroundWindow(window.hwnd) == 0 {
                    Err("Failed to switch windows".to_string())
                } else {
                    self.current = window.clone();
                    Ok(())
                }
            },
            None => Err("Left window does not exist".to_string()),
        }
    }
}

macro_rules! switch_to_direction {
    ($self:ident, $direction:ident) => {

    pub fn switch_window(&mut $self) -> Result<(), String> {
        match &$self.$direction{
            Some(window) => unsafe {
                if SetForegroundWindow(window.hwnd) == 0 {
                    Err("Failed to switch windows".to_string())
                } else {
                    self.current = window.clone();
                    Ok(())
                }
            },
            None => Err("Window does not exist".to_string()),
        }
    }
    };
}

unsafe extern "system" fn enum_windows_proc(hwnd: HWND, lparam: LPARAM) -> i32 {
    let mut buffer: [WCHAR; 1024] = [0; 1024];
    let window_manager = &mut *(lparam as *mut WindowManager);
    let length = GetWindowTextW(hwnd, buffer.as_mut_ptr(), buffer.len() as i32);

    if length > 0 && IsWindowVisible(hwnd) != 0 && hwnd != window_manager.current.hwnd {
        window_manager.count += 1;
        let window = Window::new(hwnd, window_manager.count);
        if window.placement.showCmd != SW_HIDE as u32
            && window.placement.showCmd != SW_SHOWMINIMIZED as u32
        {
            println!("showcmd {}", window.placement.showCmd);
            println!("sw_minimize {}", SW_MINIMIZE);
            window_manager.set_window(window);
        }
    }
    1
}

