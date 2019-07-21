#![allow(non_snake_case)]

extern crate ncursesw;
extern crate ascii;

use ascii::*;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let color_pair = ColorPair::default();
    let attrs = Attributes::default() | color_pair;

    let ascii_charA = AsciiChar::A;
    let ascii_charB = AsciiChar::B;

    let chtype_charA = ChtypeChar::new(ascii_charA) | attrs;
    let chtype_charB = ChtypeChar::new(ascii_charB) | attrs;

    addch(chtype_charB)?;

    r#move(Origin { y: 0, x: 0 })?;

    insch(chtype_charA)?;

    mvaddstr(Origin { y: 3, x: 0 }, "hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
