/*
    examples/wbkgd.rs

    Copyright (c) 2019, 2020 Stephen Whittle  All rights reserved.

    Permission is hereby granted, free of charge, to any person obtaining a copy
    of this software and associated documentation files (the "Software"),
    to deal in the Software without restriction, including without limitation
    the rights to use, copy, modify, merge, publish, distribute, sublicense,
    and/or sell copies of the Software, and to permit persons to whom
    the Software is furnished to do so, subject to the following conditions:
    The above copyright notice and this permission notice shall be included
    in all copies or substantial portions of the Software.
    THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
    IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
    FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
    THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
    LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
    FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
    IN THE SOFTWARE.
*/

extern crate ncursesw;
extern crate ascii;

use std::{str::FromStr, error::Error};

use ascii::*;

use ncursesw::*;
use ncursesw::normal::*;
use ncursesw::shims::ncurses::ACS_CKBOARD;

fn main() -> Result<(), Box<dyn Error>> {
    let win = initscr()?;

    let color_pair = if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        ColorPair::new(1, Colors::new(yellow, blue))?
    } else {
        ColorPair::default()
    };

    let attrs = Attribute::Bold | color_pair;

    wbkgd(win, ChtypeChar::from_chtype(ACS_CKBOARD()) | attrs)?;

    let ascii_str = AsciiString::from_ascii("hit <return> to continue")?;
    let chtype_str = ChtypeString::from_ascii_string(&ascii_str) | attrs;

    let origin = Origin { y: 3, x: 0 };

    mvwaddchstr(win, origin, &chtype_str)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
