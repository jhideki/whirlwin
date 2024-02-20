use winapi::shared::windef::{HWND, RECT};
use winapi::um::winuser::{GetWindowRect, GetWindowTextLengthW, GetWindowTextW};
pub struct Window {
    pub hwnd: HWND,
    pub rect: RECT,
}

impl Window {
    pub fn new(hwnd: HWND) -> Self {
        let mut rect: RECT = RECT::default();
        unsafe {
            GetWindowRect(hwnd, &mut rect);
        }
        Self { hwnd, rect }
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

