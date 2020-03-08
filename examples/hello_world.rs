/*
    examples/hello_world.rs

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

use std::convert::TryFrom;
use ncursesw::*;

fn main() {
    if let Err(source) = menu_routine() {
        eprintln!("error: {}", source);
    }
}

fn menu_routine() -> Result<(), NCurseswError> {
    let hello_world = "Hello World!!!";

    // initialize ncurses.
    initscr()?;
    // print "hello world!!!"
    mvaddstr(Origin { y: LINES() / 2, x: (COLS() / 2) - i32::try_from(hello_world.len())? }, hello_world)?;
    // print it on to the real screen.
    refresh()?;
    //wait for user input
    getch()?;
    // end ncurses.
    endwin()
}
