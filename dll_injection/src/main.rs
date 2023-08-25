mod process;
mod tests;

fn main() 
{
    crate::process::get_process_id_by_name("notepad".to_string());
    unsafe 
    {
        match crate::process::open_process(6252)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

