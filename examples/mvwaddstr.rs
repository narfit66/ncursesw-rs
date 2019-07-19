extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    use_default_colors()?;

    let ascii_str = "Testing..Testing..1..2..3..";

    let add_length = ascii_str.len() as i32;
    let win_size = getmaxyx(win)?;

    let mut origin = Origin { y: (win_size.lines - 1) / 2, x: ((win_size.columns - 1) / 2) - (add_length / 2) };

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    mvwaddstr(win, origin, &ascii_str)?;

    origin.y += 3;
    origin.x = ((win_size.columns - 1) / 2) - 12;

    wmove(win, origin)?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
