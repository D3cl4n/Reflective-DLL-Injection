use crate::process::open_process;

// Test case for open_process
pub unsafe fn open_process_test(process_id: u32)
{
    let result: bool;
    unsafe 
    {
        match open_process(process_id)
        {
            Some(_process_handle) => result = true,
            None => result = false
        }
    }   

    if result
    {
        println!("[+] Open process generic test passed");
    }

    else if !result
    {
        println!("[!] Open process generic test failed");
    }
}