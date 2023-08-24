mod process;
mod tests;

// not specifying pub means private by default
unsafe fn run_test_cases()
{
    tests::open_process_test(6252);
}

fn main() 
{
    unsafe 
    {
        run_test_cases();
        match process::open_process(6252)
        {
            Some(process_handle) => println!("[+] Opened process handle: {:?}", process_handle),
            None => println!("[!] Failed to open process handle"),
        }
    }
}

