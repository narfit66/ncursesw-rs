extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;
    let mut s = "Testing..1..2..3..";

    waddstr(win, &s)?;

    s = "Testing..";

    wmove(win, Origin { y: 0, x: 0 })?;
    winsstr(win, &s)?;

    mvwaddstr(win, Origin { y: 3, x: 0 }, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
