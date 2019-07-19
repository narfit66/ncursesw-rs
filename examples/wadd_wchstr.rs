extern crate ncursesw;

use std::str::FromStr;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    use_default_colors()?;

    let mut color_pair = ColorPair::default();

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;

        color_pair = ColorPair::new(1, Colors::new(yellow, blue))?;
    }

    let attrs = Attribute::Bold | color_pair;

    let complex_str = ComplexString::from_str("Testing..Testing..1..2..3..", &attrs, &color_pair)?;

    wadd_wchstr(win, &complex_str)?;

    wmove(win, Origin { y: 3, x: 0 })?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
