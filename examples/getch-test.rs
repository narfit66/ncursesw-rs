/*
    examples/getch-test.rs

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

use ncursesw::*;

fn main() {
    if let Err(source) = menu_test() {
        eprintln!("error: {}", source);
    }
}

fn menu_test() -> Result<(), NCurseswError> {
    // initialize ncurses.
    initscr()?;
    cbreak()?;
    noecho()?;

    // set our cursor to invisible.
    curs_set(CursorType::Invisible)?;

    // set keypad on so we can recieve function keys etc.
    keypad(stdscr(), true)?;

    let mut origin = Origin { y: 5, x: 5 };

    mvaddstr(origin, "Press <F1> to Exit or any other key to see result")?;

    origin.y += 2;
    mvaddstr(origin, "Result Type =")?;
    origin.x = 19;

    loop {
        let result_type = getch()?;

        r#move(origin)?;
        clrtoeol()?;

        match result_type {
            CharacterResult::Key(result_type_as_keybinding)  => {            // received a keybinding.
                if result_type_as_keybinding == KeyBinding::FunctionKey(1) { // received function key 1.
                    break
                } else {
                    mvaddstr(origin, &format!("{:?}, KeyBinding = {:?}", result_type, result_type_as_keybinding))?;
                }
            },
            CharacterResult::Character(result_type_as_char)  => {            // received a character.
                mvaddstr(origin, &format!("{:?}, char = {}", result_type, result_type_as_char))?;
            }
        };

        refresh()?;
    }

    // end ncurses.
    endwin()
}
