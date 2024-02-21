use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{
    GetWindowInfo, GetWindowPlacement, GetWindowRect, GetWindowTextLengthW, GetWindowTextW,
    WINDOWINFO, WINDOWPLACEMENT,
};
// Used to store window rect without padding
pub struct Rect {
    pub right: i32,
    pub left: i32,
    pub top: i32,
    pub bottom: i32,
}
pub struct Window {
    pub hwnd: HWND,
    pub rect: Rect,
    pub placement: WINDOWPLACEMENT,
    pub info: WINDOWINFO,
}

impl Window {
    pub fn new(hwnd: HWND) -> Self {
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
            top: rect.top - info.cxWindowBorders as i32,
            bottom: rect.bottom + info.cxWindowBorders as i32,
        }
    }

    pub fn get_title(&self) -> Option<String> {
        let length = unsafe { GetWindowTextLengthW(self.hwnd) };
        if length == 0 {
            return None;
        }
        let mut buffer: Vec<u16> = vec![0; (length + 1) as usize];

        let actual_length = unsafe { GetWindowTextW(self.hwnd, buffer.as_mut_ptr(), length + 1) };
        if actual_length == 0 {
            return None;
        }

        let title = String::from_utf16(buffer.as_slice()).unwrap();
        Some(title)
    }

    pub fn print_title(&self) {
        if let Some(title) = self.get_title() {
            println!("Window title: {}", title);
        } else {
            println!("No title");
        }
    }
}

