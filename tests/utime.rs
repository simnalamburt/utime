extern crate utime;

#[cfg(windows)] extern crate winapi;
#[cfg(windows)] extern crate kernel32;

use std::fs::File;
use utime::*;
use std::time::{SystemTime, UNIX_EPOCH};

fn as_secs(v: std::io::Result<SystemTime>) -> u64 {
    v.unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

#[test]
fn test_set_times() {
    use std::fs::metadata;

    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    // utime
    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();

    // Check if set_file_times works correctly
    let meta = metadata(path).unwrap();
    let atime = as_secs(meta.accessed());
    let mtime = as_secs(meta.modified());
    assert_eq!(atime, 1_000_000);
    assert_eq!(mtime, 1_000_000_000);
}

#[test]
fn test_get_times() {
    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();
    assert_eq!(get_file_times(path).unwrap(), (1_000_000, 1_000_000_000));
}
