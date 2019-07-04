ncursesw
========

This is a *fat* wrapper around the ncurses TUI library, it's purpose is too make the ncurses functionally safe to use but please be aware that there are certian functions within the native ncurses library that are inheritenly unsafe and under certian circumstances can cause unpredictable results, these functions have been implemented and can be called but have been marked as *depreciated* as of version 0.1.2.

This crate supports ncurses ABI 6 and above.

There are actually three layers of ncurses functions exposed within this library, the raw `unsafe` functions that are generated with `bindgen` which are available in `ncursesw::shims::bindings`, a layer above this which again are mainly `unsafe` but protect the calling code to a certian degree with assetions, these can be found in `ncursesw::shims::{ncurses, npanels, nmouse}`. Last but not least there are the safe (within the limits of ncurses itself) functions in `ncursesw` and `ncursesw::panel` which retain thier original ncurses names but have been rustified.

There is a companion wrapper around this library `ncursesw-win`(https://crates.io/crates/ncursesw-win) which abstracts away the raw pointers that ncurses uses and functions in a more safe way, however a knowledge of how ncurses works is advised to use the true power of this library.

## Requirements

At the moment this crate has only been tested on 64-bit Linux (Linux Mint 19.1) so should work on debian 64-bit flavors of Linux but this is as yet unproven.

## Inclusion

```
[dependencies]
ncursesw = "0.1"
```
Or to use the latest git version
```
[dependencies]
ncursesw = { git = "https://github.com/narfit66/ncursesw-rs" }
```

## Building

This crate has *only* been tested on Debian based x86_64 Linux (see above).

You need to have the ncurses library installed on your system, included in the root directory of this project are two bin script which will download ncurses library ABI 6.1 `ncurses-install` (this will download into `/usr/local/src`) and `ncurses-compile` which will compile and install ncurses into `/usr/lib` with wide character and extended color support.

The compiled library will be built in the `target` directory.

```
cargo build
```

## How to Use

```
extern crate ncursesw;

use ncursesw::*;
```

This library follows the basic principles that are used when using ncurses with `C`, it supports the standard ascii function (the add function seem to support unicode characters out of the box in ABI 6 if not earlier), ascii characters with attributes and/or color (chtype), wide characters (wchar_t/wint_t) and complex characters with attributes and color (cchar_t).

All features are supported as of ncurses ABI 6.1 including extended color pairs, soft labels and ripoff lines, i would suggest examining ncurses maintainer Thomas E. Dickey online documentation (https://invisible-island.net/ncurses/man/ncurses.3x.html) and read the excelent reference book `Dan Gookin's Guide to Ncurses Programming` to gain an understanding in how to use this library.
