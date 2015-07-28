//! Missing utime function for Rust
//!
//! Standard library of Rust doesn't provide the way to set atime/mtime of a file. This crate
//! provides stable way to change a file's last modification and access time.

#[cfg(unix)] extern crate libc;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

use std::path::Path;
use std::io;

/// Changes the timestamps for a file's last modification and access time.
///
/// The file at the path specified will have its last access time set to
/// `accessed` and its modification time set to `modified`. The times specified
/// should be in seconds.
pub fn set_file_times<P: AsRef<Path>>(path: P, accessed: u64, modified: u64) -> io::Result<()> {
    utime(path, accessed, modified)
}

#[cfg(unix)]
fn utime<P: AsRef<Path>>(path: P, atime: u64, mtime: u64) -> io::Result<()> {
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
fn utime<P: AsRef<Path>>(path: P, atime: u64, mtime: u64) -> io::Result<()> {
    use std::fs::OpenOptions;
    use std::os::windows::prelude::*;
    use winapi::{FILETIME, DWORD};
    use kernel32::SetFileTime;

    let f = try!(OpenOptions::new().write(true).open(path));
    let atime = to_filetime(atime);
    let mtime = to_filetime(mtime);

    // FILETIME is a count of 100ns intervals, and there are 10^7 of these in a second
    fn to_filetime(seconds: u64) -> FILETIME {
        let seconds = seconds * 10_000_000;
        FILETIME {
            dwLowDateTime: seconds as DWORD,
            dwHighDateTime: (seconds >> 32) as DWORD,
        }
    }

    let ret = unsafe { SetFileTime(f.as_raw_handle() as *mut _, 0 as *const _, &atime, &mtime) };
    if ret != 0 {
        Ok(())
    } else {
        Err(io::Error::last_os_error())
    }
}
