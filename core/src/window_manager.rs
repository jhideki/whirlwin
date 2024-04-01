use crate::callbacks::enum_windows_proc;
use crate::window::Window;

use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use windows::Win32::Foundation::{HWND, LPARAM};
use windows::Win32::UI::WindowsAndMessaging::{
    CloseWindow, EnumWindows, GetForegroundWindow, SendMessageW, SetForegroundWindow, SetWindowPos,
    HWND_BOTTOM, SWP_NOMOVE, SWP_NOSIZE, WM_CLOSE,
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
    EndListener,
    SetCurrent,
}

pub struct WindowManager {
    pub current: Window,
    left: Option<Window>,
    right: Option<Window>,
    below: Option<Window>,
    above: Option<Window>,
    next: Option<Window>,
    window_stack: Vec<Window>, // keep track of prevous windows
    stack_bottom: Option<HWND>,
    pub count: i32, // corresponds to order in window struct
    receiver: Receiver<WindowManagerMessage>,
    pub blacklist: HashSet<String>, //window titles that will not be managed
}
impl WindowManager {
    pub fn new(receiver: Receiver<WindowManagerMessage>) -> WindowManager {
        let mut names = HashSet::new();
        names.insert("Windows Input Experience".to_string());
        names.insert("Program Manager".to_string());
        names.insert("Settings".to_string());
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
            receiver,
            blacklist: names,
        }
    }

    pub fn start(mut self) {
        while let Ok(message) = self.receiver.recv() {
            match message {
                WindowManagerMessage::SetWindows => self.set_windows(),
                WindowManagerMessage::CloseWindow => self.close_window(),
                WindowManagerMessage::SwitchToNext => self.switch_to_next(),
                WindowManagerMessage::SwitchToPrevious => self.switch_to_previous(),
                WindowManagerMessage::SwitchToDirection(direction) => {
                    self.switch_to_direction(direction)
                }
                WindowManagerMessage::ClearWindows => self.clear_windows(),
                WindowManagerMessage::SetCurrent => self.set_current(),
                WindowManagerMessage::EndListener => break,
            }
        }
    }

    fn set_current(&mut self) {
        self.current = unsafe { Window::new(GetForegroundWindow(), 0) };
    }

    pub fn set_window(&mut self, window: Window) {
        window.print_title();
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
        let window = match direction {
            Direction::Left => self.left.take(),
            Direction::Right => self.right.take(),
            Direction::Below => self.below.take(),
            Direction::Above => self.above.take(),
        };

        if let Some(window) = window {
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

    fn close_window(&mut self) {
        unsafe {
            let _ = SendMessageW(self.current.hwnd, WM_CLOSE, None, None);
        }
        if let Some(window) = &self.next {
            self.current = window.to_owned();
            self.set_windows();
        }
    }

    #[allow(dead_code)]
    pub fn print_windows(&mut self) {
        println!("current");
        let _ = &self.current.print_title();
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
        println!("");
        unsafe {
            let _ = EnumWindows(
                Some(enum_windows_proc),
                LPARAM(self as *mut WindowManager as isize),
            );
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
                if let Err(e) = SetWindowPos(
                    self.current.hwnd,
                    HWND_BOTTOM,
                    0,
                    0,
                    0,
                    0,
                    SWP_NOMOVE | SWP_NOSIZE,
                ) {
                    println!("Switch to next {}: ", e);
                }
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
        self.set_windows();
    }
}
