#[cfg(unix)]
extern crate libc;

use std::path::Path;
use std::io;

#[cfg(unix)]
pub fn set_file_times<P: AsRef<Path>>(path: P, atime: u64, mtime: u64) -> io::Result<()> {
    use std::os::unix::prelude::*;
    use std::ffi::CString;
    use libc::{timeval, time_t, c_char, c_int};
    extern {
        fn utimes(name: *const c_char, times: *const timeval) -> c_int;
    }

    let path = try!(CString::new(path.as_ref().as_os_str().as_bytes()));
    let atime = timeval { tv_sec: atime as time_t, tv_usec: 0, };
    let mtime = timeval { tv_sec: mtime as time_t, tv_usec: 0, };
    let times = [atime, mtime];
    let ret = unsafe { utimes(path.as_ptr(), times.as_ptr()) };

    if ret == 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}

#[cfg(windows)]
pub fn set_file_times<P: AsRef<Path>>(path: P, atime: u64, mtime: u64) -> io::Result<()> {
    use std::fs::OpenOptions;
    use std::os::windows::prelude::*;
    use winapi::{FILETIME, DWORD};
    use kernel32::SetFileTime;

    let f = try!(OpenOptions::new().write(true).open(p));
    let atime = to_filetime(atime);
    let mtime = to_filetime(mtime);
    let ret = unsafe { SetFileTime(f.as_raw_handle() as *mut _, 0 as *const _, &atime, &mtime) };

    if ret != 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }

    // FILETIME is a count of 100ns intervals, and there are 10^7 of these in a second
    fn to_filetime(seconds: u64) -> FILETIME {
        let seconds = seconds * 10_000_000;
        FILETIME {
            dwLowDateTime: seconds as DWORD,
            dwHighDateTime: (seconds >> 32) as DWORD,
        }
    }
}
