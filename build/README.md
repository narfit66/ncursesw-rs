## Notes

The `bindings.rs` contained in this directory is the output from `bindgen` generation in the `build.rs` source and is used so that this crate maybe compile for `docs.rs` as the external library NCurses is not part of the standard configuration of the `docs.rs` build environment. It can be forced by compiling this crate with the `docs-rs` feature using the following `cargo build --features docs-rs`.

The current version was generated against NCurses version 6.1.20180127.
