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

    add_wch(complex_char)?;

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
