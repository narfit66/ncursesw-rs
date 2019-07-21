extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attributes::default();

    let mut complex_char = ComplexChar::from_char('B', &attrs, &color_pair0)?;
    let mut origin = Origin { y: LINES() / 2, x: COLS() / 2 };

    mvadd_wch(origin, complex_char)?;

    complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;

    mvins_wch(origin, complex_char)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    mvaddstr(origin, "hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
