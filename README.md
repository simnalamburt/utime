utime [![crates-i][]][crates-a] [![travis-i][]][travis-a] [![appveyor-i][]][appveyor-a]
========
A missing utime function for Rust. [Documentation][]

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

[Documentation]: https://simnalamburt.github.io/utime/
[crates-i]: https://img.shields.io/crates/v/utime.svg
[crates-a]: https://crates.io/crates/utime
[travis-i]: https://travis-ci.org/simnalamburt/utime.svg?style=flat
[travis-a]: https://travis-ci.org/simnalamburt/utime
[appveyor-i]: https://ci.appveyor.com/api/projects/status/wl66f4br7efxw23a/branch/master?svg=true
[appveyor-a]: https://ci.appveyor.com/project/simnalamburt/utime/branch/master
