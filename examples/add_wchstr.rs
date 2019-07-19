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
    attrs = attrs | color_pair;

    let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair)?;

    add_wchstr(&complex_str)?;

    r#move(Origin { y: 3, x: 0 })?;
    addstr("hit <return> to continue ")?;

    refresh()?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
