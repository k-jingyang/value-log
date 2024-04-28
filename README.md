# value-log

Generic value log implementation for key-value separated storage, inspired by RocksDB's BlobDB [[1]](#footnotes) and implemented in safe, stable Rust.

> This crate is intended as a building block for key-value separated LSM storage.
> You probably want to use https://github.com/fjall-rs/fjall instead.

## Features

- Thread-safe API
- 100% safe & stable Rust
- Supports generic KV-index structures (LSM-tree, ...)
- Built-in per-blob compression (LZ4)
- In-memory blob cache for hot data

Keys are limited to 65536 bytes, values are limited to 2^32 bytes.

## Stable disk format

The disk format will be stable from 1.0.0 (oh, the dreaded 1.0.0...) onwards. Any breaking change after that will result in a major bump.

## License

All source code is licensed under MIT OR Apache-2.0.

All contributions are to be licensed as MIT OR Apache-2.0.

## Footnotes

[1] https://github.com/facebook/rocksdb/wiki/BlobDB
