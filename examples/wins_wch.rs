extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attributes::default();

    let mut complex_char = ComplexChar::from_char('B', &attrs, &color_pair0)?;
    wadd_wch(win, complex_char)?;

    complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;

    wmove(win, Origin { y: 0, x: 0 })?;
    wins_wch(win, complex_char)?;

    mvwaddstr(win, Origin { y: 3, x: 0 }, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
