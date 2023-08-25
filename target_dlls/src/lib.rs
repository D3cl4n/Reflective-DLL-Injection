extern crate winapi;

use winapi::um::winnt::LPSTR;
use winapi::um::winuser::{MessageBoxA, HWND_DESKTOP, MB_OK};

#[no_mangle] //making sure the function name is not optimized or changed by compiler
pub extern "C" fn payload_entry() { //specify extern C for C calling conventions ensuring compatibility with other langs
    let msg = "Injected";
    let caption = "DLL";
    unsafe
    {
        MessageBoxA(HWND_DESKTOP, msg.as_ptr() as LPSTR, caption.as_ptr() as LPSTR, MB_OK);
    }
}