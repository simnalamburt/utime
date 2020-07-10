# utime changelog
## 0.3.1

* Update dependencies: winapi 0.2, kernel32-sys 0.2 → winapi 0.3
* Use winapi dependency only in Windows, use libc dependency only in
  Unix-like target.

## 0.3.0

* Timestamps are now i64 (previously u64) seconds from the Unix epoch, the same
  as in `std::os::unix::fs::MetadataExt` and `utimes(2)`.
