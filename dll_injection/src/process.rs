extern crate winapi;
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub unsafe fn open_process(process_id: u32) -> u32
{
    // PROCESS_QUERY_INFORMATION -> flag for querying information about process
    // PROCESS_VM_READ -> flag to allow reading of process memory
    let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, 0, process_id);
    return 0;
}