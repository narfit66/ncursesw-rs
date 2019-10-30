/*
    examples/wbkgrnd.rs

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
use ncursesw::extend::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    if has_colors() {
        start_color()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
        let mut attrs = Attributes::default();
        attrs.set_bold(true);

        match std::char::from_u32(0x20) {
            Some(c) => {
                let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
                wbkgrnd(win, background_char)?;
            },
            None    => waddstr(win, "unable to convert to character!")?
        }
    } else {
        waddstr(win, "terminal has no color support!!!")?;
    }

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    let origin = Origin { y: 3, x: 2 };

    mvwaddstr(win, origin, "hit <return> to continue ")?;
    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
