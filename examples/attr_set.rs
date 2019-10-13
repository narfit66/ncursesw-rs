extern crate ncursesw;

use std::error::Error;
use ncursesw::*;
use ncursesw::extend::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    if has_colors() {
        start_color()?;

        use_default_colors()?;

        let yellow = Color::Dark(BaseColor::Yellow);
        let blue = Color::Dark(BaseColor::Blue);

        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;

        let attrs0 = Attributes::default() | Attribute::Dim;
        let attrs1 = Attributes::default() | Attribute::Bold;

        let complex_char = ComplexChar::from_char('A', &attrs0, &color_pair0)?;

        attr_set(attrs1, color_pair1)?;
        add_wch(complex_char)?;

        match attr_get()? {
            AttributesColorPairSet::Normal(s)   => {
                addstr("\n\nNormal attributes and color pair...\n\n")?;
                addstr(&format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                addstr(&format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                addstr(&format!("attributes.color_pair={:?}", s.color_pair()))?;
            },
            AttributesColorPairSet::Extended(s) => {
                addstr("\n\nExtended attributes and color pair...\n\n")?;
                addstr(&format!("attributes.is_bold={}\n", s.attributes().is_bold()))?;
                addstr(&format!("attributes.is_dim={}\n", s.attributes().is_dim()))?;
                addstr(&format!("attributes.color_pair={:?}", s.color_pair()))?;
            }
        }
    } else {
        addstr("terminal has no color support!!!")?;
    }

    addstr("\n\nhit <return> to continue ")?;

    refresh()?;

    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
