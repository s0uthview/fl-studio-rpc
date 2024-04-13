extern crate winapi;

use sysinfo::Pid;

use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::winuser::{GetWindowTextW, GetDesktopWindow, IsWindowVisible};
use winapi::um::processthreadsapi::OpenProcess;
use winapi::um::handleapi::CloseHandle;
use winapi::um::winnt::PROCESS_QUERY_INFORMATION;
use winapi::um::winuser::GetForegroundWindow;
use winapi::um::errhandlingapi::GetLastError;
use winapi::shared::minwindef::DWORD;

pub fn find_window(p: Pid) {
    let pid = p.as_u32();
    let process_handle = open_process(pid);
    
    match process_handle {
        Some(handle) => {
            let window_title = get_window_title(handle);
            match window_title {
                Some(title) => println!("Window title: {}", title),
                None => println!("Failed to retrieve window title."),
            }
            unsafe { CloseHandle(handle) };
        },
        None => println!("Failed to open process with PID: {}", pid),
    }
}

fn open_process(pid: DWORD) -> Option<winapi::um::winnt::HANDLE> {
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_INFORMATION, 0, pid);
        if process_handle.is_null() {
            println!("Failed to open process. Error code: {}", GetLastError());
            return None;
        }
        Some(process_handle)
    }
}

fn get_window_title(process_handle: winapi::um::winnt::HANDLE) -> Option<String> {
    let mut window_title = vec![0u16; 512];
    unsafe {
        if IsWindowVisible(GetDesktopWindow()) == 0 {
            return None;
        }

        let result = GetWindowTextW(GetForegroundWindow(), window_title.as_mut_ptr(), window_title.len() as i32);

        if result != 0 {
            let title_length = result as usize;
            window_title.truncate(title_length);
            Some(OsString::from_wide(&window_title).to_string_lossy().into_owned())
        } else {
            None
        }
    }
}