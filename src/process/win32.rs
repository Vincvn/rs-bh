use std::ffi::OsString;

#[cfg(target_os = "windows")]
use anyhow::Result;
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
use windows_sys::Win32::System::Threading::{OpenProcess, PROCESS_ACCESS_RIGHTS, PROCESS_TERMINATE, TerminateProcess};
pub fn kill_by_name(name: &str)->Vec<Result<()>>{
    pid_by_name(name).into_iter().map(|pid|{
        kill(pid)
    }).collect::<Vec<Result<()>>>()
}

pub unsafe fn process_handle(pid: u32, a: PROCESS_ACCESS_RIGHTS) -> HANDLE {
    let process_handle = OpenProcess(a, 0, pid);
    process_handle
}

pub fn kill(pid: u32) -> Result<()>{
    unsafe {
        let process_handle = process_handle(pid, PROCESS_TERMINATE);

        if !process_handle.is_null() {
            TerminateProcess(process_handle, 1);
            CloseHandle(process_handle);
            return Ok(())
        } else {
            return Err(anyhow::anyhow!(format!("Failed to open process pid: {}", pid)));
        }
    }
}

pub fn pid_by_name(name: &str) -> Vec<u32> {
    let pname: OsString = name.into();
    let pids : Vec<u32> = sysinfo::System::new_all().processes_by_name(pname.as_os_str()).map(|process| {
        process.pid().as_u32()
    }).collect();
    pids
}