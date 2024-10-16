#![allow(deprecated, reason = "tests for deprecated functions")]

use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};
use utime::{get_file_times, set_file_times};

fn as_secs(v: std::io::Result<SystemTime>) -> u64 {
    v.unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs()
}

fn check_utime(path: &str) {
    use std::fs::metadata;

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
fn test_set_times() {
    let path = "target/dummy1";

    // Test with a dummy file.
    File::create(path).unwrap();
    check_utime(path);
    std::fs::remove_file(path).unwrap();

    // Test with a dummy directory.
    std::fs::create_dir(path).unwrap();
    check_utime(path);
    std::fs::remove_dir(path).unwrap();
}

#[test]
fn test_get_times() {
    let path = "target/dummy2";

    // Create dummy file for the test
    File::create(path).unwrap();

    set_file_times(path, 1_000_000, 1_000_000_000).unwrap();
    assert_eq!(get_file_times(path).unwrap(), (1_000_000, 1_000_000_000));
}

#[test]
fn test_set_negative_time() {
    let path = "target/dummy3";

    File::create(path).unwrap();
    set_file_times(path, -36000, -36000).unwrap();
    assert_eq!(get_file_times(path).unwrap(), (-36000, -36000));
}
