extern crate utime;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

use std::fs::File;
use utime::*;

#[cfg(unix)]
#[test]
fn test_set_times() {
    use std::fs::metadata;
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

#[test]
fn test_get_times() {
    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();
    assert_eq!(get_file_times(path).unwrap(), (1_000_000, 1_000_000_000));
}
