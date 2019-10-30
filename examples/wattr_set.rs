/*
    examples/wattr_set.rs

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

        use_default_colors()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;

        let attrs0 = Attributes::default() | Attribute::Dim;
        let attrs1 = Attributes::default() | Attribute::Bold;

        let complex_char = ComplexChar::from_char('A', &attrs0, &color_pair0)?;

        wattr_set(win, attrs1, color_pair1)?;
        wadd_wch(win, complex_char)?;

        match wattr_get(win)? {
            AttributesColorPairSet::Normal(s)   => {
                waddstr(win, "\n\nNormal attributes and color pair...\n\n")?;
                waddstr(win, &format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                waddstr(win, &format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                waddstr(win, &format!("attributes.color_pair={:?}", s.color_pair()))?;
            },
            AttributesColorPairSet::Extended(s) => {
                waddstr(win, "\n\nExtended attributes and color pair...\n\n")?;
                waddstr(win, &format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                waddstr(win, &format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                waddstr(win, &format!("attributes.color_pair={:?}", s.color_pair()))?;
            }
        }
    } else {
        waddstr(win, "terminal has no color support!!!")?;
    }

    waddstr(win, "\n\nhit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
