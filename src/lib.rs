//! (Deprecated) Rust's once-missing utime function
//!
//! This crate was originally created to provide a missing utime function for Rust,
//! allowing users to set the [atime/mtime] of a file, as the Rust standard library
//! did not offer a stable method for this functionality.
//!
//! As of Rust 1.75.0, the standard library now includes [`File::set_times`], which
//! provides a stable and native way to update a file’s last modification and access
//! time.
//!
//! [atime/mtime]: https://man7.org/linux/man-pages/man3/stat.3type.html#SYNOPSIS:~:text=file%20has%20holes.%29-,st_atime,-This%20is%20the
//! [`File::set_times`]: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_times
//!
//! ### Recommendation
//! If you are using Rust 1.75.0 or later, it is recommended to use the native
//! [`File::set_times`] function instead of this crate.
//!
//! ```rust
//! use std::fs::{File, FileTimes};
//! use std::time::SystemTime;
//!
//! # fn main() -> std::io::Result<()> {
//! let times = FileTimes::new()
//!     .set_accessed(SystemTime::now())
//!     .set_modified(SystemTime::UNIX_EPOCH);
//! File::create("target/testdummy")?.set_times(times)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### For Rust <1.75.0 users
//! For projects using older Rust versions, you may still find this library useful.
//! See [documentation](https://docs.rs/utime) for the further details.
//!
//! ```rust
//! use std::fs::File;
//! use utime::*;
//!
//! # fn main() -> std::io::Result<()> {
//! File::create("target/testdummy")?;
//! set_file_times("target/testdummy", 1000000, 1000000000)?;
//!
//! let (accessed, modified) = get_file_times("target/testdummy")?;
//! assert_eq!(accessed, 1000000);
//! assert_eq!(modified, 1000000000);
//! # Ok(())
//! # }
//! ```

#![deny(missing_docs)]

#[cfg(unix)]
extern crate libc;

#[cfg(windows)]
extern crate winapi;

use std::io;
use std::path::Path;

/// Changes the timestamps for a file's last modification and access time.
///
/// The file at the path specified will have its last access time set to
/// `accessed` and its modification time set to `modified`. The times specified
/// should be in seconds from the Unix epoch.
///
/// ### Deprecated
/// Starting from Rust 1.75.0, [`File::set_times`] is now available. Please use [`File::set_times`]
/// instead.
///
/// [`File::set_times`]: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_times
#[deprecated(
    note = "Starting from Rust 1.75.0, File::set_times is now available. Please use File::set_times instead."
)]
pub fn set_file_times<P: AsRef<Path>>(path: P, accessed: i64, modified: i64) -> io::Result<()> {
    #[cfg(unix)]
    fn utime<P: AsRef<Path>>(path: P, atime: i64, mtime: i64) -> io::Result<()> {
        use libc::{c_char, c_int, time_t, timeval};
        use std::ffi::CString;
        use std::os::unix::prelude::*;
        extern "C" {
            fn utimes(name: *const c_char, times: *const timeval) -> c_int;
        }

        let path = CString::new(path.as_ref().as_os_str().as_bytes())?;
        let atime = timeval {
            tv_sec: atime as time_t,
            tv_usec: 0,
        };
        let mtime = timeval {
            tv_sec: mtime as time_t,
            tv_usec: 0,
        };
        let times = [atime, mtime];

        let ret = unsafe { utimes(path.as_ptr(), times.as_ptr()) };
        if ret == 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    #[cfg(windows)]
    fn utime<P: AsRef<Path>>(path: P, atime: i64, mtime: i64) -> io::Result<()> {
        use std::fs::OpenOptions;
        use std::os::windows::prelude::*;
        use winapi::shared::minwindef::{DWORD, FILETIME};
        use winapi::um::fileapi::SetFileTime;

        let f = OpenOptions::new()
            .write(true)
            .custom_flags(winapi::um::winbase::FILE_FLAG_BACKUP_SEMANTICS)
            .open(path)?;
        let atime = to_filetime(atime);
        let mtime = to_filetime(mtime);

        // FILETIME is a count of 100ns intervals, and there are 10^7 of these in a second
        fn to_filetime(seconds: i64) -> FILETIME {
            let intervals = seconds * 10_000_000 + 116_444_736_000_000_000;
            FILETIME {
                dwLowDateTime: intervals as DWORD,
                dwHighDateTime: (intervals >> 32) as DWORD,
            }
        }

        let ret = unsafe {
            SetFileTime(
                f.as_raw_handle() as *mut _,
                std::ptr::null(),
                &atime,
                &mtime,
            )
        };
        if ret != 0 {
            Ok(())
        } else {
            Err(io::Error::last_os_error())
        }
    }

    utime(path, accessed, modified)
}

/// Retrieve the timestamps for a file's last modification and access time.
///
/// Returns `(accessed, modified)`. The times are in seconds from the Unix epoch.
///
/// ### Deprecated
///
/// Starting from Rust 1.10.0, [`Metadata::accessed`] and [`Metadata::modified`] are now available.
/// Please use [`Metadata::accessed`] and [`Metadata::modified`] instead.
///
/// [`Metadata::accessed`]: https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.accessed
/// [`Metadata::modified`]: https://doc.rust-lang.org/stable/std/fs/struct.Metadata.html#method.modified
#[deprecated(
    note = "Starting from Rust 1.10.0, Metadata::accessed and Metadata::modified are now available. Please use Metadata::accessed and Metadata::modified instead."
)]
pub fn get_file_times<P: AsRef<Path>>(path: P) -> io::Result<(i64, i64)> {
    #[cfg(unix)]
    fn utime<P: AsRef<Path>>(path: P) -> io::Result<(i64, i64)> {
        use std::fs::metadata;
        use std::os::unix::fs::MetadataExt;

        let meta = metadata(path)?;
        Ok((meta.atime(), meta.mtime()))
    }

    #[cfg(windows)]
    fn utime<P: AsRef<Path>>(path: P) -> io::Result<(i64, i64)> {
        use std::fs::OpenOptions;
        use std::os::windows::prelude::*;
        use winapi::shared::minwindef::FILETIME;
        use winapi::um::fileapi::GetFileTime;

        let f = OpenOptions::new().write(true).open(path)?;
        let handle = f.as_raw_handle() as *mut _;
        let mut atime = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };
        let mut mtime = FILETIME {
            dwLowDateTime: 0,
            dwHighDateTime: 0,
        };

        let ret = unsafe { GetFileTime(handle, std::ptr::null_mut(), &mut atime, &mut mtime) };
        if ret == 0 {
            return Err(io::Error::last_os_error());
        }

        // FILETIME is a count of 100ns intervals, and there are 10^7 of these in a second
        fn to_seconds(ft: FILETIME) -> i64 {
            let lo = ft.dwLowDateTime as u64;
            let hi = (ft.dwHighDateTime as u64) << 32;
            let intervals = (lo + hi) as i64 - 116_444_736_000_000_000;

            intervals / 10_000_000
        }

        Ok((to_seconds(atime), to_seconds(mtime)))
    }

    utime(path)
}
