/*
    examples/screen-test.rs

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

use std::{convert::TryFrom, env, io};

use ncursesw::{*, shims::ncurses::{ACS_ULCORNER, ACS_LLCORNER, ACS_URCORNER, ACS_LRCORNER, ACS_HLINE, ACS_VLINE}};

macro_rules! result { ($type: ty) => { Result<$type, NCurseswError> } }

fn main() {
    if let Err(source) = main_routine() {
        eprintln!("error: {}", source);
    }
}

fn main_routine() -> result!(()) {
    let term = &env::var("TERM").expect("$TERM is undefined!!!");

    // create a screen using stdout and stdin for output and input.
    let screen = newterm(Some(term), &io::stdout().lock(), &io::stdin().lock())?;

    // make the screens cursor invisible.
    curs_set_sp(screen, CursorType::Invisible)?;
    // switch echoing off.
    noecho_sp(screen)?;

    // create a window on our screen.
    let window = newwin_sp(screen, Size::default(), Origin::default())?;

    // extract the box drawing characters for the box drawing type.
    let left_side   = ChtypeChar::from_chtype(ACS_VLINE());
    let right_side  = ChtypeChar::from_chtype(ACS_VLINE());
    let top_side    = ChtypeChar::from_chtype(ACS_HLINE());
    let bottom_side = ChtypeChar::from_chtype(ACS_HLINE());
    let upper_left  = ChtypeChar::from_chtype(ACS_ULCORNER());
    let upper_right = ChtypeChar::from_chtype(ACS_URCORNER());
    let lower_left  = ChtypeChar::from_chtype(ACS_LLCORNER());
    let lower_right = ChtypeChar::from_chtype(ACS_LRCORNER());

    // create a border on the inital window.
    wborder(window, left_side, right_side, top_side, bottom_side, upper_left, upper_right, lower_left, lower_right)?;

    // the text we are going to output.
    let line1 = "If the doors of perception were cleansed every thing would appear to man as it is: Infinite.";
    let line2 = "For man has closed himself up, till he sees all things thro' narrow chinks of his cavern.";
    let line3 = "Press any key to exit";

    // get the window's size.
    let window_size = getmaxyx(window)?;

    // calculate the initial origin for line 1.
    let mut origin = Origin { y: (window_size.lines / 2) - 2, x: calc_x_axis(line1, window_size)? };

    // output our lines centered on the x-axis.
    mvwaddstr(window, origin, line1)?;
    origin.y += 1;
    origin.x = calc_x_axis(line2, window_size)?;
    mvwaddstr(window, origin, line2)?;
    origin.y += 2;
    origin.x = calc_x_axis(line3, window_size)?;
    mvwaddstr(window, origin, line3)?;

    // wait for the user to press a key.
    wgetch(window)?;

    // free window.
    delwin(window)?;

    // end NCurses and free the screen.
    endwin_sp(screen)?;
    delscreen(screen);

    Ok(())
}

fn calc_x_axis(line: &str, window_size: Size) -> result!(i32) {
    Ok((window_size.columns / 2) - (i32::try_from(line.len())? / 2))
}
