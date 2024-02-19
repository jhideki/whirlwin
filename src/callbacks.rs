use std::ptr::null_mut;
use winapi::shared::minwindef::LPARAM;
use winapi::shared::minwindef::WPARAM;
use winapi::um::winuser;

pub unsafe extern "system" fn keyboard_callback(
    code: i32,
    w_param: WPARAM,
    l_param: LPARAM,
) -> isize {
    if code >= 0 {
        println!("{}", w_param);
    }
    winuser::CallNextHookEx(null_mut(), code, w_param, l_param)
}
pub unsafe extern "system" fn shell_hook_callback(
    code: i32,
    _w_param: WPARAM,
    l_param: LPARAM,
) -> isize {
    if code == winuser::HSHELL_WINDOWCREATED {
        println!("a new window has been created {}",);
    }
    winuser::CallNextHookEx(null_mut(), code, _w_param, l_param)
}

