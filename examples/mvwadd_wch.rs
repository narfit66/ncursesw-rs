extern crate ncursesw;

use std::str::FromStr;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

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

    let win_size = getmaxyx(win)?;
    let mut origin = Origin { y: (win_size.lines - 1) / 2, x: (win_size.columns - 1) / 2 };

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    mvwadd_wch(win, origin, complex_char)?;

    origin.y += 3;
    origin.x = ((win_size.columns - 1) / 2) - 12;

    wmove(win, origin)?;
    waddstr(win, "hit <return> to continue ")?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
