## Issues with Travis CI

When compiling under Travis CI `curses_version()` and `ncurses_version()` are reporting different version numbers!!!

$ cargo run --example ncursesw-version
   Compiling ncursesw v0.3.3 (/home/travis/build/narfit66/ncursesw-rs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.52s
     Running `target/debug/examples/ncursesw-version`

curses_version:  ncurses 6.0.20160213
ncurses_version: 6.1.20180127

The command "cargo run --example ncursesw-version" exited with 0.

## Menu module

The menu module is just for reference at the moment and does not work correctly.
