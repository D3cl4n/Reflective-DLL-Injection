mod process;
mod tests;

fn main() 
{
    let process: &str = "notepad.exe";
    let mut process_pid: u32 = 0;
    match crate::process::get_process_id_by_name(process)
    {
        Some(pid) => process_pid = pid,
        None => println!("[!] Failed to get PID from process name"),
    }

    unsafe 
    {
        match crate::process::open_process(process_pid)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }

        let dll_path: &str = "target\\debug\\target_dlls.dll";
        match crate::process::get_dll_size(dll_path)
        {
            Some(length) => println!("yay"),
            None => println!("no yay"),
        }
    }
}

