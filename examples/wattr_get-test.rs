/*
    examples/wattr_get.rs

    Copyright (c) 2019-2022 Stephen Whittle  All rights reserved.

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

use ncursesw::{*, normal::*};

fn main() {
    if let Err(source) = main_routine() {
        println!("error: {}", source);
    }
}

fn main_routine() -> Result<(), NCurseswError> {
    let stdscr = initscr()?;

    if has_colors() {
        start_color()?;

        use_default_colors()?;

        let yellow = Color::new(ColorPalette::Yellow);
        let blue = Color::new(ColorPalette::Blue);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;

        let attrs0 = Attributes::default();
        let attrs1 = Attributes::default().set_bold(true);

        wattr_set(stdscr, attrs1, color_pair1)?;
        waddstr(stdscr, "Test string printed with attr1 and color_pair1")?;

        let retrived_attrs_colorpair = wattr_get(stdscr)?.unwrap_as_normal();

        wattr_set(stdscr, attrs0, color_pair0)?;

        waddstr(stdscr, "\n\nNormal attributes and color pair of `attrs1`...\n\n")?;
        waddstr(stdscr, format!("retrived_attrs.attributes={:?}\n", retrived_attrs_colorpair.attributes()))?;
        waddstr(stdscr, format!("retrived_attrs.is_normal={}\n", retrived_attrs_colorpair.attributes().is_normal()))?;
        waddstr(stdscr, format!("retrived_attrs.is_bold={}\n", retrived_attrs_colorpair.attributes().is_bold()))?;
        waddstr(stdscr, format!("retrived_attrs.color_pair={:?}", retrived_attrs_colorpair.color_pair()))?;
    } else {
        waddstr(stdscr, "terminal has no color support!!!")?;
    }

    waddstr(stdscr, "\n\nhit <return> to continue ")?;
    wgetch(stdscr)?;

    delwin(stdscr)?;

    endwin()
}
