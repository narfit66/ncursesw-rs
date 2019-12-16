ncursesw [![Build Status](https://travis-ci.com/narfit66/ncursesw-rs.svg?branch=master)](https://travis-ci.com/narfit66/ncursesw-rs) [![Crates.io](https://img.shields.io/crates/v/ncursesw.svg)](https://crates.io/crates/ncursesw)
========

This is a *fat* wrapper around the ncurses TUI library, it's purpose is too make the ncurses functionally safe to use but please be aware that there are certian functions within the native ncurses library that are inheritenly unsafe and under certian circumstances can cause unpredictable results, these functions have been implemented and can be called but have been marked as *depreciated* as of version 0.1.2/0.1.3.

This crate supports ncurses ABI 6.1 and above.

There are actually three layers of ncurses functions exposed within this library, the raw `unsafe` functions that are generated with `bindgen` which are available in `ncursesw::shims::bindings`, a layer above this which again are mainly `unsafe` but protect the calling code to a certian degree with assetions (some functions will also have a slight rust wrapping (for example functions returning a raw pointer) but are in the whole left as per the bindgen version), these can be found in `ncursesw::shims::{ncurses, npanels, nmouse, nmenu, nform}`. Last but not least there are the safe (within the limits of ncurses itself) functions in `ncursesw` and `ncursesw::{*, panels, mouse, menu, form}` which retain thier original ncurses names but have been rustified.

ncurses modules implemented and rustified so far are the main ncurses library, mouse (version 2 only), panels, menu and forms.

There is a companion crate which acts as a wrapper around this crate [ncursesw-win](https://crates.io/crates/ncursesw-win) which abstracts away the raw pointers that ncurses uses and functions in a more safe way, however a knowledge of how ncurses works is advised to use the true power of this library.

## Requirements

At the moment this crate has only been tested on 64-bit Linux (Linux Mint 19.1) so should work on debian 64-bit flavors of Linux but this is as yet unproven.

## Inclusion

```
[dependencies]
ncursesw = "0.4"
```
Or to use the latest git version
```
[dependencies]
ncursesw = { git = "https://github.com/narfit66/ncursesw-rs" }
```

## Building

This crate has *only* been tested on Debian based x86_64 Linux (see above).

You need to have the ncurses library (ABI 6 and above) installed on your system, included in the root directory of this project are two bin script which will download ncurses library ABI 6.1 `ncurses-install` (this will download into `/usr/local/src`) and `ncurses-compile` which will compile and install ncurses into `/usr/lib` with wide character and extended color support.

The compiled library will be built in the `target` directory.

```
cargo build
```

## Custom Build

Environment variables are used by `build.rs`:

If set, `NCURSESW_RUSTC_LINK_LIB` will be used for the `cargo:rustc-link-lib` setting and `NCURSESW_RUSTC_FLAGS` will be used for the `cargo:rustc-flags` setting.

## Features

By default this crate will be compiled so that the following NCurses functions `getch()`, `mvgetch()`, `mvwgetch()`, `wgetch()`, `get_wch()`, `mvget_wch()`, `mvwget_wch()` and `wget_ch()` will pass a `KEY_RESIZE` on event of the terminal being resized or a `KEY_EVENT` back to the client code as `KeyBinding::ResizeEvent` and `KeyBinding::Event` respectivly. The follwing setting in the client crates `Cargo.toml` will cause this crate to be compiled so that they will be passed back as `NCurseswError::KeyResize` and `NCurseswError::Event` error types instead.

```
[features]
key_resize_as_error = ["ncursesw/key_resize_as_error"]
key_event_as_error = ["ncursesw/key_event_as_error"]
```

## How to Use

```
extern crate ncursesw;

use ncursesw::*;
```

This library follows the basic principles that are used when using ncurses with `C`, it supports the standard ascii functions (the add function seem to support unicode characters out of the box in ABI 6 if not earlier), ascii characters with attributes and/or color (`chtype`), wide characters (`wchar_t`/`wint_t`) and complex characters with attributes and color (`cchar_t`).

Color pairs and attributes are dealt with in two modules. The `normal` module deals with the standard `ansi` color pairs defined internally within ncurses as `short_t/i16` and the `extend` module is for extended color pairs that are defined internally within ncurses as `i32`. Because the `normal` color pairs are actually an attribute within ncurses both modules also implement there own `attribute` and `attributes` types.

To use attributes and color pairs
```
use ncursesw::normal::*; // for 'ansi' color pairs and attributes...
use ncursesw::extend::*; // or for 'extended' color pairs and attributes.
```

To use the panels functions
```
use ncursesw::panels::*;
```

To use the mouse functions
```
use ncursesw::mouse::*;
```

To use the menu functions
```
use ncursesw::menu::*;
```

To use the form functions
```
use ncursesw::form::*;
```

All features are supported as of ncurses ABI 6.1 including extended color pairs, soft labels, ripoff lines, panels, mouse, menus and forms, i would suggest examining ncurses maintainer Thomas E. Dickey [online documentation](https://invisible-island.net/ncurses/man/ncurses.3x.html) and also the [panels](https://invisible-island.net/ncurses/man/panel.3x.html), [mouse](https://invisible-island.net/ncurses/man/curs_mouse.3x.html), [menu](https://invisible-island.net/ncurses/man/menu.3x.html) and [form](https://invisible-island.net/ncurses/man/form.3x.html) documentation, if you get the chance have a read of the book `Dan Gookin's Guide to Ncurses Programming` by well i guessing here but i'm thinking it's Dan Gookin, this is a good primer to gain an understanding in how to use this library. In both cases you will need a basic knowlege of 'C'.

Alternativly have a look at the crate [ncursesw-win](https://crates.io/crates/ncursesw-win) which wraps this crate safely by not exposing the ncurses library raw pointers and by encapsulating them in formalised structures.

## Documentation

Please use `cargo doc --open` for this crate for the time being!.

## License

Licensed under the MIT license, see [LICENSE](https://github.com/narfit66/ncursesw-rs/blob/master/LICENSE)
