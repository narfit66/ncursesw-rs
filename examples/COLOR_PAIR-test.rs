/*
    examples/COLOR_PAIR-test.rs

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

#![allow(non_snake_case)]
#![allow(deprecated)]

use std::str::FromStr;
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

        let yellow = Color::new(ColorPalette::from_str("yellow")?);
        let blue = Color::new(ColorPalette::from_str("blue")?);
        let green = Color::new(ColorPalette::from_str("blue")?);
 
        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
        let color_pair2 = ColorPair::new(2, Colors::new(yellow, green))?;
        let color_pair3 = ColorPair::new(3, Colors::new(blue, yellow))?;
        let color_pair4 = ColorPair::new(4, Colors::new(blue, green))?;
        let color_pair5 = ColorPair::new(5, Colors::new(green, yellow))?;
        let color_pair6 = ColorPair::new(6, Colors::new(green, blue))?;

        addstr(color_pair_info(color_pair0))?;
        addstr(color_pair_info(color_pair1))?;
        addstr(color_pair_info(color_pair2))?;
        addstr(color_pair_info(color_pair3))?;
        addstr(color_pair_info(color_pair4))?;
        addstr(color_pair_info(color_pair5))?;
        addstr(color_pair_info(color_pair6))?;
    } else {
        addstr("terminal has no color support!!!")?;
    }

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(stdscr)?;
    endwin()
}

fn color_pair_info(color_pair: ColorPair) -> String {
    format!("color pair {} : attribute 0b{:032b}\n", color_pair.number(), COLOR_PAIR(i32::from(color_pair.number())))
}
