ncursesw-rs
===========

This is a *fat* wrapper around the ncurses TUI library, it's purpose is too make the ncurses functionally safe to use but please be aware that there are certian functions within ncurses that are inheritenly unsafe and under certian circumstances can cause unpredictable results.

There are actually three layers of ncurses functions exposed within this library, the raw `unsafe` functions that are generated with `bindgen` are are available in `ncursesw::shims::bindings`, a layer above this which again are mainly `unsafe` but protect the calling code to a certian degree with assetions, these can be found in `ncursesw::shims::{ncurses, npanels, nmouse}`. Last but not least there are the safe (mainly) functions in `ncursesw` and `ncursesw::panel` which retain thier original ncurses names but have been rustified.

There is a companion wrapper around this library called ncurses-win-rs(https://github.com/narfit66/ncursesw-win-rs) which abstracts away the raw pointers that ncurses uses and functions in a more safe way, however a knowledge of how ncurses works is advised to use the true power of this library.

## Building

The compiled library will be built in the `target` directory.

```
cargo build
```

## How to Use

This library follows the basic principles that are used when using ncurses with `C`, it supports the standard ansii function (the add function seem to support unicode characters out of the box in version 6), ansi charcaters with attributes (chtype), wide characters (wchar_t/wint_t) and complex characters (cchar_t).

All features are supported as of ncurses 6.1 including extended color pairs, soft labels and ripoff lines, i would suggest examining ncurses maintainer Thomas E. Dickey online documentation(https://invisible-island.net/ncurses/man/ncurses.3x.html) and read the excelent reference book `Dan Gookin's Guide to Ncurses Programming` to gain an understanding in how to use this library.
