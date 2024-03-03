use crate::window::Window;

use winapi::shared::minwindef::LPARAM;
use winapi::shared::windef::HWND;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::{
    CloseWindow, EnumWindows, GetClassNameW, GetForegroundWindow, GetWindow, GetWindowTextW,
    IsWindowVisible, SendMessageW, SetForegroundWindow, SetWindowPos, GW_HWNDNEXT, HWND_BOTTOM,
    SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_MINIMIZE, SW_SHOWMINIMIZED, WM_CLOSE,
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
        if self.left.is_none() && window.rect.right <= self.current.rect.left {
            self.left = Some(window);
        } else if self.right.is_none() && window.rect.left >= self.current.rect.right {
            self.right = Some(window);
        } else if self.above.is_none() && window.rect.bottom <= self.current.rect.top {
            self.above = Some(window);
        } else if self.below.is_none() && window.rect.top >= self.current.rect.bottom {
            self.below = Some(window);
        } else if self.behind.is_none() && window.monitor == self.current.monitor {
            self.behind = Some(window)
        }
    }

    pub fn close_window(&mut self) {
        unsafe {
            SendMessageW(self.current.hwnd, WM_CLOSE, 0, 0);
        }
        if let Some(window) = &self.behind {
            self.current = window.to_owned();
            self.behind = None;
            self.set_windows();
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
        println!("current");
        &self.current.print_title();
        println!("");

        print!("left");
        match &self.left {
            Some(window) => window.print_title(),
            None => println!("left window does not exist"),
        }
        println!("");

        println!("");
        print!("right");
        match &self.right {
            Some(window) => window.print_title(),
            None => println!("window does not exist"),
        }

        println!("");
        print!("above");
        match &self.above {
            Some(window) => window.print_title(),
            None => println!(" window does not exist"),
        }

        println!("");
        print!("below");
        match &self.below {
            Some(window) => window.print_title(),
            None => println!(" window does not exist"),
        }

        println!("");
        print!("behind");
        match &self.behind {
            Some(window) => window.print_title(),
            None => println!("window does not exist"),
        }
    }

    pub fn set_windows(&mut self) {
        unsafe {
            EnumWindows(Some(enum_windows_proc), self as *mut _ as LPARAM);
        }
        //Self::print_windows(self);
    }

    pub fn clear_windows(&mut self) {
        self.left = None;
        self.right = None;
        self.above = None;
        self.below = None;
        self.behind = None;
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
    pub fn switch_to_behind(&mut self) {
        if let Some(window) = self.behind.take() {
            window.print_title();
            unsafe {
                if SetForegroundWindow(window.hwnd) == 0 {
                    println!("Failed to switch to behind");
                } else {
                    SetWindowPos(
                        self.current.hwnd,
                        HWND_BOTTOM,
                        0,
                        0,
                        0,
                        0,
                        SWP_NOMOVE | SWP_NOSIZE,
                    );
                    self.clear_windows();
                    self.current = window;
                    self.set_windows();
                }
            }
        }
    }
}

#[macro_export]
macro_rules! switch_to_direction {
    ($window_manager:expr, $direction:ident) => {
        if let Some(window) = $window_manager.$direction.take() {
            if SetForegroundWindow(window.hwnd) == 0 {
                println!("Failed to switch windows");
            }
            $window_manager.clear_windows();
            $window_manager.current = window;
            $window_manager.set_windows();
            println!("Switch to window {}", stringify!($direction));
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
            if let Some(title) = window.get_title() {
                if title != "Windows Input Experience" {
                    /*println!("");
                    println!(
                        "window title chars: {:?} window string {}",
                        title.chars().map(|c| c as u32).collect::<Vec<_>>(),
                        title
                    );*/
                    window_manager.set_window(window);
                }
            }
        }
    }
    1
}

