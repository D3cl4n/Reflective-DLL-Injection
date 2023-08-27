mod process;
mod tests;

fn main() 
{
    let process: &str = "notepad";
    crate::process::get_process_id_by_name(process);
    unsafe 
    {
        match crate::process::open_process(6252)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

