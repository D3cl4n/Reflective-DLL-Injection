#[cfg(feature = "dll")]
extern crate winapi;

use winapi::um::winuser::{MessageBoxA, HWND_DESKTOP, MB_OK};

fn main()
{
    let msg = "hello world!";
    let caption = "Simple DLL";

    unsafe 
    {
        MessageBoxA(HWND_DESKTOP, msg.as_ptr() as LPSTR, caption.as_ptr() as LPSTR, MB_OK);
    }
}