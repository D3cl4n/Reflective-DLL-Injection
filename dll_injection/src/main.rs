mod process;
mod tests;

fn main() 
{
    #[cfg(feature = "dll")]
    {
        println!("[+] Compiling dll: src/dll/hello_world.rs");
    }

    unsafe 
    {
        match crate::process::open_process(6252)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

