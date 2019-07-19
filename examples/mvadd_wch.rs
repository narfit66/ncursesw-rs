extern crate ncursesw;

use std::str::FromStr;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    let mut color_pair = ColorPair::default();

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        color_pair = ColorPair::new(1, Colors::new(yellow, blue))?;
    }

    let mut attrs = Attributes::default();
    attrs.set_bold(true);

    let complex_char = ComplexChar::from_char('A', &attrs, &color_pair)?;

    let mut origin = Origin { y: LINES() / 2, x: COLS() / 2 };

    mvadd_wch(origin, complex_char)?;

    origin.y += 3;
    origin.x = (COLS() / 2) - 12;

    r#move(origin)?;
    addstr("hit <return> to continue ")?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
