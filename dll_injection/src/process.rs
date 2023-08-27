extern crate winapi;
extern crate sysinfo;

use std::ptr;
use sysinfo::{ProcessExt, System, SystemExt};
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};

pub fn get_process_id_by_name(process_name: &str)
{
    println!("[+] Finding process PID for : {:?}", process_name);
    let s = System::new_all();

    for process in s.processes_by_exact_name(process_name)
    {
        println!("[+] Process name: {:?}", process.name());
        println!("[+] Nodepad PID: {:?}", process.pid());
    }
}

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