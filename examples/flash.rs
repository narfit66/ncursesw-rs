extern crate ncursesw;

use std::error::Error;
use ncursesw::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let default_border = ChtypeChar::from_chtype(0);
    border(default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    refresh()?;

    std::thread::sleep(std::time::Duration::from_millis(1000));

    flash()?;

    let origin = Origin { y: 3, x: 2};

    mvaddstr(origin, "hit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
