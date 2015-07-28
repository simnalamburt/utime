extern crate utime;

use std::fs::{File, metadata};
use utime::set_file_times;

#[test]
fn test_utime() {
    use std::os::unix::fs::MetadataExt;

    let path = "target/testdummy";

    // Create dummy file for the test
    File::create(path).unwrap();

    // utime
    set_file_times(path, 1000000, 1000000000).unwrap();

    // Check if set_file_times functions correctly
    let meta = metadata(path).unwrap();
    assert_eq!(meta.atime(), 1000000);
    assert_eq!(meta.mtime(), 1000000000);
}
