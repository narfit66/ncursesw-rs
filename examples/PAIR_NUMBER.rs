#![allow(non_snake_case)]

extern crate ncursesw;

use std::str::FromStr;
use std::error::Error;
use ncursesw::*;
use ncursesw::normal::*;

fn main() -> Result<(), Box<Error>> {
    let h = initscr()?;

    if has_colors() {
        start_color()?;

        let yellow = Color::from_str("yellow")?;
        let blue = Color::from_str("blue")?;
 
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
        let attrs = Attribute::Bold | color_pair1;

        addstr(&format!("color pair raw value {:?}\n", color_pair1))?;
        addstr(&format!("attributes raw value {:?}\n\n", attrs))?;
        addstr(&format!("color pair from attributes is {:?}", PAIR_NUMBER(attrs)))?;
    } else {
        addstr("terminal has no color support!!!")?;
    }

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
