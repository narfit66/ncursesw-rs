/*
    examples/border.rs

    Copyright (c) 2019 Stephen Whittle  All rights reserved.

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

use std::error::Error;
use ncursesw::*;
use shims::ncurses::{
    ACS_VLINE, ACS_HLINE, ACS_ULCORNER,
    ACS_URCORNER, ACS_LLCORNER, ACS_LRCORNER
};

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let ls = ChtypeChar::from_chtype(ACS_VLINE());
    let rs = ChtypeChar::from_chtype(ACS_VLINE());
    let ts = ChtypeChar::from_chtype(ACS_HLINE());
    let bs = ChtypeChar::from_chtype(ACS_HLINE());
    let tl = ChtypeChar::from_chtype(ACS_ULCORNER());
    let tr = ChtypeChar::from_chtype(ACS_URCORNER());
    let bl = ChtypeChar::from_chtype(ACS_LLCORNER());
    let br = ChtypeChar::from_chtype(ACS_LRCORNER());

    border(ls, rs, ts, bs, tl, tr, bl, br)?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
