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

    let add_length = complex_str.len() as i32;
    let win_size = getmaxyx(win)?;

    let mut origin = Origin { y: (win_size.lines - 1) / 2, x: ((win_size.columns - 1) / 2) - (add_length / 2) };

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    mvwadd_wchstr(win, origin, &complex_str)?;

    origin.y += 3;
    origin.x = ((win_size.columns - 1) / 2) - 12;

    wmove(win, origin)?;
    waddstr(win, "hit <return> to continue ")?;

    wrefresh(win)?;

    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
