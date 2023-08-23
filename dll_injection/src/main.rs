mod process;

fn main() 
{
    println!("Hello, world!");
    unsafe 
    {
        match process::open_process(5128)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

