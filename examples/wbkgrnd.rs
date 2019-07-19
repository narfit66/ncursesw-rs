extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::extend::*;

fn main() -> Result<(), Box<Error>> {
    let win = initscr()?;

    if has_colors() {
        start_color()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
        let mut attrs = Attributes::default();
        attrs.set_bold(true);

        match std::char::from_u32(0x20) {
            Some(c) => {
                let background_char = ComplexChar::from_char(c, &attrs, &color_pair1)?;
                wbkgrnd(win, background_char)?;
            },
            None    => waddstr(win, "unable to convert to character!")?
        }
    } else {
        waddstr(win, "terminal has no color support!!!")?;
    }

    let default_border = ChtypeChar::from_chtype(0);
    wborder(win, default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    let origin = Origin { y: 3, x: 2 };

    mvwaddstr(win, origin, "hit <return> to continue ")?;
    wgetch(win)?;

    delwin(win)?;
    endwin()?;

    Ok(())
}
