utime - Deprecated [![version]][crates.io]
========
This crate was originally created to provide a missing utime function for Rust,
allowing users to set the [atime/mtime] of a file, as the Rust standard library
did not offer a stable method for this functionality.

As of Rust 1.75.0, the standard library now includes [`File::set_times`], which
provides a stable and native way to update a file’s last modification and access
time.

[atime/mtime]: https://man7.org/linux/man-pages/man3/stat.3type.html#SYNOPSIS:~:text=file%20has%20holes.%29-,st_atime,-This%20is%20the
[`File::set_times`]: https://doc.rust-lang.org/stable/std/fs/struct.File.html#method.set_times

### Recommendation
If you are using Rust 1.75.0 or later, it is recommended to use the native
[`File::set_times`] function instead of this crate.

```rust
use std::fs::{File, FileTimes};
use std::io::Result;
use std::time::SystemTime;

fn main() -> Result<()> {
    let times = FileTimes::new()
        .set_accessed(SystemTime::now())
        .set_modified(SystemTime::UNIX_EPOCH);
    File::options().write(true).open("dest")?.set_times(times)?;
    Ok(())
}
```

### For Rust <1.75.0 users
For projects using older Rust versions, you may still find this library useful.
See [documentation](https://docs.rs/utime) for the further details.

```toml
[dependencies]
utime = "0.3"
```
```rust
use std::fs::File;
use std::io::Result;
use utime::*;

fn main() -> Result<()> {
    File::create("target/testdummy")?;
    set_file_times("target/testdummy", 1000000, 1000000000)?;

    let (accessed, modified) = get_file_times("target/testdummy")?;
    assert_eq!(accessed, 1000000);
    assert_eq!(modified, 1000000000);
    Ok(())
}
```

&nbsp;

--------
*utime* is primarily distributed under the terms of both the [MIT
license] and the [Apache License (Version 2.0)]. See [COPYRIGHT] for details.

[version]: https://badgen.net/crates/v/utime
[crates.io]: https://crates.io/crates/utime
[MIT license]: LICENSE-MIT
[Apache License (Version 2.0)]: LICENSE-APACHE
[COPYRIGHT]: COPYRIGHT
