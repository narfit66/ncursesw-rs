/*
    examples/mvwins_wch.rs

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
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attributes::default();

    let mut complex_char = ComplexChar::from_char('B', &attrs, &color_pair0)?;
    let mut origin = Origin { y: LINES() / 2, x: COLS() / 2 };

    mvwadd_wch(win, origin, complex_char)?;

    complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;

    mvwins_wch(win, origin, complex_char)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    mvwaddstr(win, origin, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
