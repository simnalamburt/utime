# utime changelog
## 0.3.2
* Marked all functions as deprecated. See [README.md] for the further details.

[README.md]: README.md

## 0.3.1

* Update dependencies: winapi 0.2, kernel32-sys 0.2 → winapi 0.3
* Use winapi dependency only in Windows, use libc dependency only in
  Unix-like target.

## 0.3.0

* Timestamps are now i64 (previously u64) seconds from the Unix epoch, the same
  as in [`std::os::unix::fs::MetadataExt`] and [`utimes(2)`].

[`std::os::unix::fs::MetadataExt`]: https://doc.rust-lang.org/stable/std/os/unix/fs/trait.MetadataExt.html
[`utimes(2)`]: https://man7.org/linux/man-pages/man2/utimes.2.html
