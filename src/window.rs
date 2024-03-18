use windows::Win32::Foundation::{HWND, RECT};
use windows::Win32::Graphics::Gdi::{MonitorFromWindow, HMONITOR, MONITOR_DEFAULTTOPRIMARY};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowInfo, GetWindowPlacement, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    WINDOWINFO, WINDOWPLACEMENT,
};
// Used to store window rect without padding
#[derive(Clone)]
pub struct Rect {
    pub right: i32,
    pub left: i32,
    pub top: i32,
    pub bottom: i32,
}
#[derive(Clone)]
pub struct Window {
    pub hwnd: HWND,
    pub rect: Rect,
    pub placement: WINDOWPLACEMENT,
    pub info: WINDOWINFO,
    pub order: i32, // foreground window has order of 0
    pub monitor: HMONITOR,
}

impl Window {
    pub fn new(hwnd: HWND, order: i32) -> Self {
        let mut placement: WINDOWPLACEMENT = WINDOWPLACEMENT::default();
        let mut info: WINDOWINFO = WINDOWINFO::default();
        unsafe {
            GetWindowPlacement(hwnd, &mut placement);
            GetWindowInfo(hwnd, &mut info);
        }
        Self {
            hwnd,
            rect: Self::set_rect(hwnd, info),
            placement,
            info,
            order,
            monitor: unsafe { MonitorFromWindow(hwnd, MONITOR_DEFAULTTOPRIMARY) },
        }
    }

    fn set_rect(hwnd: HWND, info: WINDOWINFO) -> Rect {
        let mut rect: RECT = RECT::default();
        unsafe {
            GetWindowRect(hwnd, &mut rect);
        }
        Rect {
            right: rect.right - info.cxWindowBorders as i32,
            left: rect.left + info.cxWindowBorders as i32,
            top: rect.top + info.cxWindowBorders as i32,
            bottom: rect.bottom - info.cxWindowBorders as i32,
        }
    }

    pub fn get_title(&self) -> Option<String> {
        let length = unsafe { GetWindowTextLengthW(self.hwnd) };
        if length == 0 {
            return None;
        }
        let mut buffer = vec![0u16; (length + 1) as usize];

        let actual_length = unsafe { GetWindowTextW(self.hwnd, &mut buffer) };
        if actual_length == 0 {
            return None;
        }

        let title = String::from_utf16(buffer.as_slice()).unwrap();
        Some(title.trim_end_matches(char::from(0)).to_owned())
    }

    pub fn print_title(&self) {
        if let Some(title) = self.get_title() {
            println!("Window title: {}", title);
        } else {
            println!("No title");
        }
    }
}

