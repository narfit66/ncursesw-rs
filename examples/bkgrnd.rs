extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::extend::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

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
                bkgrnd(background_char)?;
            },
            None    => addstr("unable to convert to character!")?
        }
    } else {
        addstr("terminal has no color support!!!")?;
    }

    let default_border = ChtypeChar::from_chtype(0);
    border(default_border, default_border, default_border, default_border, default_border, default_border, default_border, default_border)?;

    let origin = Origin { y: 3, x: 2 };

    mvaddstr(origin, "hit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
