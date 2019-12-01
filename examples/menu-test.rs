/*
    examples/menu-test.rs

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

use ncursesw::{*, menu::*};

fn main() {
    if let Err(source) = menu_test() {
        eprintln!("error: {}", source);
    }
}

fn menu_test() -> Result<(), NCurseswError> {
    // Initialize curses.
    initscr()?;
    start_color()?;
    cbreak()?;
    noecho()?;
    curs_set(CursorType::Invisible)?;
    keypad(stdscr(), true)?;

    // Create items.
    let mut items = vec!();

    items.push(new_item("Choice 1", "Choice 1 description")?);
    items.push(new_item("Choice 2", "Choice 2 description")?);
    items.push(new_item("Choice 3", "Choice 3 description")?);
    items.push(new_item("Choice 4", "Choice 4 description")?);
    items.push(new_item("Exit", "Exit description")?);

    // Crate menu.
    let my_menu = new_menu(&items)?;

    let mut menu_opts = MenuOptions::default();
    menu_opts.set_show_description(true);

    menu_opts_off(my_menu, menu_opts)?;

    let my_menu_win = newwin(Size { lines: 9, columns: 18 }, Origin { y: 4, x: 4 })?;
    keypad(my_menu_win, true)?;

    // Set main window and sub window.
    set_menu_win(Some(my_menu), Some(my_menu_win))?;
    let my_menu_win_der_win = derwin(my_menu_win, Size { lines: 5, columns: 0 }, Origin { y: 2, x: 2 })?;
    set_menu_sub(Some(my_menu), Some(my_menu_win_der_win))?;

    // Set menu mark to the string " * ".
    set_menu_mark(my_menu, " * ")?;

    // Print a border around the main window.
    r#box(my_menu_win, ChtypeChar::from(0), ChtypeChar::from(0))?;
    mvaddstr(Origin { y: LINES() - 3, x: 0 }, "Press <Enter> to see the option selected")?;
    mvaddstr(Origin { y: LINES() - 2, x: 0 }, "F1 to exit")?;
    refresh()?;

    // Post the menu.
    post_menu(my_menu)?;
    wrefresh(my_menu_win)?;

    let mut ch = getch()?;

    while ch != CharacterResult::Key(KeyBinding::FunctionKey(1)) {
        let _ = match ch {
            CharacterResult::Key(KeyBinding::UpArrow)   => menu_driver(my_menu, MenuRequest::UpItem)?,
            CharacterResult::Key(KeyBinding::DownArrow) => menu_driver(my_menu, MenuRequest::DownItem)?,
            CharacterResult::Character('\n')            => { //Enter
                r#move(Origin { y: 20, x: 0 })?;
                clrtoeol()?;
                mvaddstr(Origin { y: 20, x: 0 }, &format!("Item selected is : {}", item_name(current_item(my_menu)?)?))?;
                pos_menu_cursor(my_menu)?;
                None
            },
            _   => None
        };

        wrefresh(my_menu_win)?;

        ch = getch()?;
    }

    unpost_menu(my_menu)?;

    // Free menu.
    free_menu(my_menu)?;

    // free items.
    for item in items.iter().rev() {
        free_item(*item)?;
    }

    delwin(my_menu_win_der_win)?;
    delwin(my_menu_win)?;

    endwin()
}
