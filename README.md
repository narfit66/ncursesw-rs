ncursesw [![Crates.io](https://img.shields.io/crates/v/ncursesw.svg)](https://crates.io/crates/ncursesw) [![Build Status](https://travis-ci.com/narfit66/ncursesw-rs.svg?branch=master)](https://travis-ci.com/narfit66/ncursesw-rs) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/narfit66/ncursesw-rs/blob/master/LICENSE) ![Lines of Code](https://tokei.rs/b1/github/narfit66/ncursesw-rs?category=code)
========

This is a **fat** wrapper around the [NCurses TUI library](https://github.com/mirror/ncurses), it's purpose is too make the native library functionally safe to use, however please be aware that there are a number of functions within the native library that under certain circumstances can cause undefined behaviour, these functions have been implemented and can be called but have been marked as *deprecated* with appropriate notes.

There are actually three layers of NCurses functions exposed within this library, the raw `extern "C"` functions that are generated with [bindgen](https://crates.io/crates/bindgen) which are available in `ncursesw::shims::bindings`.

A layer above this which are mainly `unsafe` but protect the calling code to a certain degree with assertions (some functions will also have a slight rust wrapping (for example functions returning a raw pointer) but are on the whole left as per the `bindgen` layer), these can be found in `ncursesw::shims::{ncurses, npanels, nmouse, nmenu, nform}` and you can consider this layer as the equivalent of a `-sys` crate for the NCurses library.

Last but not least there are the safe (within the limits of NCurses itself) functions in `ncursesw` and `ncursesw::{panels, mouse, menu, form}` which retain there original NCurses names but have been rustified.

There is a companion crate [ncursesw-win](https://crates.io/crates/ncursesw-win) which acts as a wrapper around this crate and encapsulates the raw pointers that NCurses uses and provides a higher level of functionality.

## Requirements

At the moment this crate has only been tested on 64-bit Linux (Linux Mint 19.1) so should work on Debian 64-bit flavors of Linux but this is as yet unproven.

## Recomendations and Native Library Support

I whould recommend using version `0.6` and above of this crate as the crates API has pretty much stabilized at this point. In addition from `0.6.2` the native NCurses library (currently `v6.3 patch 20211021`) is locally compiled and statically linked into this crate so there is no dependency on the version of NCurses (ABI or otherwise) that is currently install via the distribution/operating system.

## Inclusion

```
[dependencies]
ncursesw = "0.6"
```
Or to use the latest git version
```
[dependencies]
ncursesw = { git = "https://github.com/narfit66/ncursesw-rs" }
```

## Building

As noted above this crate has *only* been tested on Debian based x86_64 Linux.

The NCurses library is locally compiled and statically linked when this crate is built with all features required (wide character and extended color support) along with the menu, form and panels libraries.

The compiled crate will be built in the `target` directory.

```
cargo build
```

## Features

By default this crate will be compiled so that the following NCurses functions `getch()`, `mvgetch()`, `mvwgetch()`, `wgetch()`, `get_wch()`, `mvget_wch()`, `mvwget_wch()` and `wget_wch()` will pass a `KEY_RESIZE` on event of the terminal being resized back to the client code as `KeyBinding::ResizeEvent`. The follwing setting in the client code crates `Cargo.toml` will cause this crate to be compiled so that they will be passed back as `NCurseswError::KeyResize` error types instead.

```
[features]
key_resize_as_error = ["ncursesw/key_resize_as_error"]
```

## How to Use

```
extern crate ncursesw;

use ncursesw::*;
```

This crate follows the basic principles that are used when using NCurses with `C`, it supports the standard ascii functions (the `add` type functions seem to support unicode characters out of the box in NCurses `v6.1.20180127` if not earlier), ascii characters with rendition (`chtype`), wide characters (`wchar_t`/`wint_t`) and complex characters with rendition (`cchar_t`).

Color pairs and attributes (rendition) are dealt with in two modules. The `normal` module deals with the standard `ansi` color pairs defined internally within NCurses as `short_t/i16` and the `extend` module is for extended color pairs that are defined internally within NCurses as `i32`. Because the `normal` color pairs are actually an attribute within NCurses both modules also implement there own `Attribute` and `Attributes` types.

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

To use wide (UTF-8) characters `setlocale()` needs to be called before the NCurses library is initialised, in the examples the [gettext-rs](https://crates.io/crates/gettext-rs) crate has been used for this purpose.

All features are supported as of NCurses v6.1 including ansi 8-bit (normal) colors and color pairs, extended colors and color pairs, characters and strings (with attribute/color rendition), wide (UTF-8) characters and strings, complex characters and strings, soft labels, ripoff lines, panels, mouse, menus, forms and screen functions. I would suggest examining NCurses maintainer Thomas E. Dickey [online documentation](https://invisible-island.net/ncurses/man/ncurses.3x.html) and also the [panels](https://invisible-island.net/ncurses/man/panel.3x.html), [mouse](https://invisible-island.net/ncurses/man/curs_mouse.3x.html), [menu](https://invisible-island.net/ncurses/man/menu.3x.html), [form](https://invisible-island.net/ncurses/man/form.3x.html) and [screen](https://invisible-island.net/ncurses/man/curs_sp_funcs.3x.html) documentation. If you get the chance have a read of the book `Dan Gookin's Guide to NCurses Programming` by well i guessing here but i'm thinking it's Dan Gookin, this is a good primer to gain an understanding in how to use this library. In both cases you will need a basic knowlege of 'C'.

Alternativly have a look at the crate [ncursesw-win](https://crates.io/crates/ncursesw-win) which wraps this crate with the express purpose of not exposing the NCurses library raw pointers and encapsulating them in formalised structures.

## Documentation

Documentation for this crate can be found [here](https://docs.rs/ncursesw).

## License

Licensed under the MIT license, see [LICENSE](https://github.com/narfit66/ncursesw-rs/blob/master/LICENSE)
