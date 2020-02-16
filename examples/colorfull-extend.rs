/*
    examples/colorfull-extend.rs

    Copyright (c) 2020 Stephen Whittle  All rights reserved.

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

use ncursesw::{*, extend::*};

fn main() {
    if let Err(source) = main_routine() {
        println!("error: {}", source);
    }
}

fn main_routine() -> Result<(), NCurseswError> {
    initscr()?;

    if has_colors() {
        start_color()?;

        let red = Color::new(ColorPalette::Red);
        let black = Color::new(ColorPalette::Black);
        let yellow = Color::new(ColorPalette::Yellow);

        let colors1 = Colors::new(red, black);
        let colors2 = Colors::new(yellow, black);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, colors1)?;
        let color_pair2 = ColorPair::new(2, colors2)?;

        let mut attrs = Attributes::default();

        attr_set(attrs, color_pair0)?;
        addstr("Using modern attribute setting...\n\n")?;

        attr_set(attrs, color_pair1)?;
        addstr("I am Mr. Red!\n")?;
        attr_set(attrs, color_pair2)?;
        addstr("I am Mr. Yellow!\n")?;
        attrs = attrs.set_bold(true);
        attr_set(attrs, color_pair1)?;
        addstr("I'm feeling bold!\n")?;
        attr_set(attrs, color_pair2)?;
        addstr("Me too!\n")?;
    } else {
        addstr("terminal has no color support!!!")?;
    }

    refresh()?;

    getch()?;

    endwin()
}
