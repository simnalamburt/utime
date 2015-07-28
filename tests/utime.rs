extern crate utime;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

use utime::set_file_times;

#[cfg(unix)]
#[test]
fn test_utime() {
    use std::fs::{File, metadata};
    use std::os::unix::fs::MetadataExt;

    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    // utime
    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();

    // Check if set_file_times works correctly
    let meta = metadata(path).unwrap();
    assert_eq!(meta.atime(), 1_000_000);
    assert_eq!(meta.mtime(), 1_000_000_000);
}

#[cfg(windows)]
#[test]
fn test_utime() {
    use std::io;
    use std::path::Path;
    use std::fs::File;

    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    // utime
    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();

    // Check if get_file_times works correctly
    let (atime, mtime) = get_file_times(path).unwrap();
    assert_eq!(atime, 1_000_000);
    assert_eq!(mtime, 1_000_000_000);

    fn get_file_times<P: AsRef<Path>>(path: P) -> io::Result<(u64, u64)> {
        use std::fs::OpenOptions;
        use std::os::windows::prelude::*;
        use winapi::FILETIME;
        use kernel32::GetFileTime;

        let f = try!(OpenOptions::new().write(true).open(path));
        let handle = f.as_raw_handle() as *mut _;
        let mut atime = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };
        let mut mtime = FILETIME { dwLowDateTime: 0, dwHighDateTime: 0 };

        let ret = unsafe { GetFileTime(handle, 0 as *mut _, &mut atime, &mut mtime) };
        if ret == 0 { return Err(io::Error::last_os_error()); }

        // FILETIME is a count of 100ns intervals, and there are 10^7 of these in a second
        fn to_seconds(ft: FILETIME) -> u64 {
            let lo = ft.dwLowDateTime as u64;
            let hi = (ft.dwHighDateTime as u64) << 32;
            let intervals = lo + hi - 116444736000000000;

            intervals / 10_000_000
        }

        Ok((to_seconds(atime), to_seconds(mtime)))
    }
}
