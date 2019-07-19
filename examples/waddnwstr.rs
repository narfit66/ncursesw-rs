extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    use_default_colors()?;

    let wide_str = WideString::from_str("Testing..Testing..1..2..3..");

    waddnwstr(win, &wide_str, 18)?;

    wmove(win, Origin { y: 3, x: 0})?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
