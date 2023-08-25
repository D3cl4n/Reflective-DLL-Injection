mod process;
mod tests;

fn main() 
{
    unsafe 
    {
        match crate::process::open_process(6252)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

