use crate::callbacks::enum_windows_proc;
use crate::window::Window;

use async_std::channel::Receiver;
use windows::Win32::Foundation::{HWND, LPARAM, WPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CloseWindow, EnumWindows, GetClassNameW, GetForegroundWindow, GetWindow, GetWindowTextW,
    IsWindowVisible, SendMessageW, SetForegroundWindow, SetWindowPos, GW_HWNDLAST, GW_HWNDNEXT,
    HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, SW_HIDE, SW_MINIMIZE, SW_SHOWMINIMIZED, WM_CLOSE,
};
pub enum Direction {
    Left,
    Right,
    Below,
    Above,
}
pub enum WindowManagerMessage {
    ClearWindows,
    SetWindows,
    CloseWindow,
    SwitchToNext,
    SwitchToPrevious,
    SwitchToDirection(Direction),
}

pub struct WindowManager {
    pub current: Window,
    pub left: Option<Window>,
    pub right: Option<Window>,
    pub below: Option<Window>,
    pub above: Option<Window>,
    next: Option<Window>,
    window_stack: Vec<Window>, // keep track of prevous windows
    stack_bottom: Option<HWND>,
    pub count: i32, // corresponds to order in window struct
    receiver: Receiver<WindowManagerMessage>,
}
impl WindowManager {
    pub fn new(receiver: Receiver<WindowManagerMessage>) -> WindowManager {
        let current = unsafe { Window::new(GetForegroundWindow(), 0) };
        WindowManager {
            current,
            left: None,
            right: None,
            below: None,
            above: None,
            next: None,
            window_stack: Vec::new(),
            stack_bottom: None,
            count: 0,
            receiver: receiver,
        }
    }

    pub async fn start(mut self) {
        while let Ok(message) = self.receiver.recv().await {
            match message {
                WindowManagerMessage::SetWindows => self.set_windows(),
                WindowManagerMessage::CloseWindow => self.close_window(),
                WindowManagerMessage::SwitchToNext => self.switch_to_next(),
                WindowManagerMessage::SwitchToPrevious => self.switch_to_previous(),
                WindowManagerMessage::SwitchToDirection(direction) => {
                    self.switch_to_direction(direction)
                }
                WindowManagerMessage::ClearWindows => self.clear_windows(),
            }
        }
    }

    pub fn set_window(&mut self, window: Window) {
        if self.left.is_none() && window.rect.right <= self.current.rect.left {
            self.left = Some(window);
        } else if self.right.is_none() && window.rect.left >= self.current.rect.right {
            self.right = Some(window);
        } else if self.above.is_none() && window.rect.bottom <= self.current.rect.top {
            self.above = Some(window);
        } else if self.below.is_none() && window.rect.top >= self.current.rect.bottom {
            self.below = Some(window);
        } else if self.next.is_none() && window.monitor == self.current.monitor {
            self.next = Some(window)
        }
    }

    fn switch_to_direction(&mut self, direction: Direction) {
        let mut option = None;
        match direction {
            Left => {
                option = Some(self.left.take());
            }
            Right => {
                option = Some(self.right.take());
            }
            Below => {
                option = Some(self.below.take());
            }
            Above => {
                option = Some(self.above.take());
            }
        }

        if let Some(window) = option.unwrap() {
            unsafe {
                if !SetForegroundWindow(window.hwnd).as_bool() {
                    println!("Failed to switch windows");
                }
            }
            self.clear_windows();
            self.current = window;
            self.set_windows();
        }
    }

    pub fn close_window(&mut self) {
        unsafe {
            //CloseWindow(self.current.hwnd);
            SendMessageW(self.current.hwnd, WM_CLOSE, WPARAM(0), LPARAM(0));
        }
        if let Some(window) = &self.next {
            self.current = window.to_owned();
            self.clear_windows();
            self.set_windows();
        }
    }

    /*fn print_window_info(window: &Window) {
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
    }*/
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
        print!("next");
        match &self.next {
            Some(window) => window.print_title(),
            None => println!("window does not exist"),
        }
    }

    pub fn set_windows(&mut self) {
        unsafe {
            EnumWindows(Some(enum_windows_proc), self as *mut _ as LPARAM);
        }
    }

    pub fn clear_windows(&mut self) {
        self.left = None;
        self.right = None;
        self.above = None;
        self.below = None;
        self.next = None;
        self.window_stack.clear();
        self.stack_bottom = None;
    }

    pub fn switch_to_previous(&mut self) {
        if let Some(window) = self.window_stack.pop() {
            self.next = Some(self.current.clone());
            self.current = window;
            unsafe {
                SetForegroundWindow(self.current.hwnd);
            }
        } else {
            println!("previous doesn't exist");
        }
    }

    pub fn switch_to_next(&mut self) {
        if let Some(window) = self.next.take() {
            window.print_title();
            unsafe {
                SetForegroundWindow(window.hwnd);
                SetWindowPos(
                    self.current.hwnd,
                    HWND_BOTTOM,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                );
            }
            if let Some(hwnd) = self.stack_bottom {
                if hwnd == window.hwnd {
                    println!("reached the first window again");
                    self.window_stack.clear();
                }
            }
            if self.window_stack.is_empty() {
                let window = self.current.clone();
                self.stack_bottom = Some(window.hwnd);
                self.window_stack.push(window);
            } else {
                self.window_stack.push(self.current.clone());
            }
            self.current = window;
        };
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

