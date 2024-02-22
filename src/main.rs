mod window;
mod window_manager;

use std::ptr::null_mut;
use winapi::um::winuser::VK_ESCAPE;
use winapi::um::winuser::WM_HOTKEY;

use std::io::Error;
use winapi::um::winuser::{GetMessageW, RegisterHotKey, UnregisterHotKey, MSG};
use window_manager::WindowManager;

const EXIT: i32 = 1;
const SWITCH_WINDOW: i32 = 2;
const KEY_H: u32 = 0x48;
const MOD_ALT: u32 = 0x0001;
fn main() -> Result<(), Error> {
    let mut window_manager = WindowManager::new();
    window_manager.set_windows();
    // window_manager.get_all_windows();
    window_manager.print_windows();
    unsafe {
        if RegisterHotKey(null_mut(), EXIT, 0, VK_ESCAPE as u32) == 0 {
            return Err(Error::last_os_error());
        }

        if RegisterHotKey(null_mut(), SWITCH_WINDOW, MOD_ALT, KEY_H as u32) == 0 {
            return Err(Error::last_os_error());
        }
        let mut msg: MSG = Default::default();
        while GetMessageW(&mut msg, null_mut(), 0, 0) != 0 {
            if msg.message == WM_HOTKEY {
                match msg.wParam as i32 {
                    EXIT => break,
                    SWITCH_WINDOW => match window_manager.switch_to_left() {
                        Ok(()) => {
                            println!("Switch windows");
                        }
                        Err(err) => eprintln!("Error: {}", err),
                    },
                    _ => println!("idk bru"),
                }
            }
        }
        UnregisterHotKey(null_mut(), EXIT);
        UnregisterHotKey(null_mut(), SWITCH_WINDOW);
    }
    Ok(())
}
