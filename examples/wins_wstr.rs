extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;
    let mut wide_str = WideString::from_str("Testing..1..2..3..");

    waddwstr(win, &wide_str)?;

    wide_str = WideString::from_str("Testing..");

    wmove(win, Origin { y: 0, x: 0 })?;
    wins_wstr(win, &wide_str)?;

    mvwaddstr(win, Origin { y: 3, x: 0 }, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
