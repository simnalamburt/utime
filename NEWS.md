# utime changelog

## 0.2.3

* Timestamps are now i64 (previously u64) seconds from the Unix epoch, the same
  as in `std::os::unix::fs::MetadataExt` and `utimes(2)`.
