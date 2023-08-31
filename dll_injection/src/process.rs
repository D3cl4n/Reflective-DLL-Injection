extern crate winapi;
extern crate sysinfo;

use std::ptr;
use std::fs;
use sysinfo::{ProcessExt, System, SystemExt, Pid, PidExt};
use winapi::um::processthreadsapi::{OpenProcess};
use winapi::um::memoryapi::{VirtualAllocEx};
use winapi::um::winnt::{MEM_COMMIT, MEM_RESERVE, PAGE_EXECUTE_READWRITE};
use winapi::shared::minwindef::{LPVOID};
use winapi::um::winnt::{PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};


pub fn virtual_alloc_ex(handle: u32, size: u64)
{
    println!("[+] Allocating {:?} bytes of RWX memory inside target process", size);

    unsafe
    {
        let allocated_addr = VirtualAllocEx(handle, ptr::null_mut(), size as usize, MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);
    }
}

pub fn get_dll_size(path: &str) -> Option<u64>
{
    println!("[+] Getting size of DLL: {:?}", path);
    match fs::metadata(path)
    {
        Ok(metadata) => 
        {
            println!("[+] DLL length: {:?}", metadata.len());
            return Some(metadata.len());
        }

        Err(_e) =>
        {
            return None;
        }
    }
}

pub fn get_process_id_by_name(process_name: &str) -> Option<u32>
{
    println!("[+] Finding process PID for : {:?}", process_name);
    let s = System::new_all();

    for process in s.processes_by_exact_name(process_name)
    {
        let process_id: Pid = process.pid();
        println!("[+] Process name: {:?}", process.name());
        println!("[+] Nodepad PID: {:?}", process_id.as_u32());

        return Some(process_id.as_u32() as u32);
    }

    None
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
        println!("[+] Opened process handle: {:?}", process_handle);
        Some(process_handle as u32)
    }
}