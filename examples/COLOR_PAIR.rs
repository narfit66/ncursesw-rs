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
        let green = Color::from_str("blue")?;
 
        let color_pair0 = ColorPair::default();
        let color_pair1 = ColorPair::new(1, Colors::new(yellow, blue))?;
        let color_pair2 = ColorPair::new(2, Colors::new(yellow, green))?;
        let color_pair3 = ColorPair::new(3, Colors::new(blue, yellow))?;
        let color_pair4 = ColorPair::new(4, Colors::new(blue, green))?;
        let color_pair5 = ColorPair::new(5, Colors::new(green, yellow))?;
        let color_pair6 = ColorPair::new(6, Colors::new(green, blue))?;

        addstr(&format!("color pair 0 attribute 0b{:016b}\n", COLOR_PAIR(color_pair0)))?;
        addstr(&format!("color pair 1 attribute 0b{:016b}\n", COLOR_PAIR(color_pair1)))?;
        addstr(&format!("color pair 2 attribute 0b{:016b}\n", COLOR_PAIR(color_pair2)))?;
        addstr(&format!("color pair 3 attribute 0b{:016b}\n", COLOR_PAIR(color_pair3)))?;
        addstr(&format!("color pair 4 attribute 0b{:016b}\n", COLOR_PAIR(color_pair4)))?;
        addstr(&format!("color pair 5 attribute 0b{:016b}\n", COLOR_PAIR(color_pair5)))?;
        addstr(&format!("color pair 6 attribute 0b{:016b}\n", COLOR_PAIR(color_pair6)))?;
    } else {
        addstr("terminal has no color support!!!")?;
    }

    addstr("\n\nhit <return> to continue ")?;
    getch()?;

    delwin(h)?;
    endwin()?;

    Ok(())
}
