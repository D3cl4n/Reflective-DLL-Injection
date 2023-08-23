extern crate winapi;

use std::ptr;
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub unsafe fn open_process(process_id: u32) -> Option<u32>
{
    // PROCESS_QUERY_INFORMATION -> flag for querying information about process
    // PROCESS_VM_READ -> flag to allow reading of process memory
    let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);

    if process_handle == ptr::null_mut()
    {
        None
    }

    else
    {
        Some(process_handle as u32)
    }
}