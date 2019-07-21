extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;
    let mut wide_str = WideString::from_str("Testing..1..2..3..");

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    let mut origin = Origin { y: LINES() / 2, x: (COLS() / 2) - 14 };

    mvwaddwstr(win, origin, &wide_str)?;

    wide_str = WideString::from_str("Testing..");

    mvwins_wstr(win, origin, &wide_str)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    mvwaddstr(win, origin, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
