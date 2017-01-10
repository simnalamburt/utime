utime [![Travis Build Status]][travis] [![AppVeyor Build Status]][appveyor]
========
A missing utime function for Rust.

- [API Documentation](https://simnalamburt.github.io/utime/)
- [See `utime` in crates.io](https://crates.io/crates/utime)

Standard library of Rust doesn't provide stable way to set atime/mtime of a
file. This crate provides stable way to change a file's last modification and
access time.

```toml
[dependencies]
utime = "0.1"
```
```rust
use std::fs::File;
use utime::*;

File::create("target/testdummy").unwrap();
set_file_times("target/testdummy", 1000000, 1000000000).unwrap();

let (accessed, modified) = get_file_times("target/testdummy").unwrap();
assert_eq!(accessed, 1000000);
assert_eq!(modified, 1000000000);
```

--------

[BSD 2-Clause](LICENSE.md)

[Travis Build Status]: https://travis-ci.org/simnalamburt/utime.svg?style=flat
[travis]: https://travis-ci.org/simnalamburt/utime
[AppVeyor Build Status]: https://ci.appveyor.com/api/projects/status/wl66f4br7efxw23a/branch/master?svg=true
[appveyor]: https://ci.appveyor.com/project/simnalamburt/utime/branch/master
