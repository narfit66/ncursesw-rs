extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;
    let s = "Testing..1..2..3..";

    waddstr(win, &s)?;

    wmove(win, Origin { y: 0, x: 0 })?;
    winsnstr(win, &s, 9)?;

    mvwaddstr(win, Origin { y: 3, x: 0 }, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
