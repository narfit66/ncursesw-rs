extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let color_pair0 = ColorPair::default();
    let attrs = Attributes::default();

    let mut complex_char = ComplexChar::from_char('B', &attrs, &color_pair0)?;
    add_wch(complex_char)?;

    complex_char = ComplexChar::from_char('A', &attrs, &color_pair0)?;

    r#move(Origin { y: 0, x: 0 })?;
    ins_wch(complex_char)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
