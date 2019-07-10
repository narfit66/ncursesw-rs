extern crate ncursesw;

use ncursesw::*;
use ncursesw::normal::*;

fn main() {
    if let Err(e) = main_routine() {
        let _ = endwin();
        println!("error: {}", e);
    }
}

fn main_routine() -> Result<(), NCurseswError> {
    initscr()?;
    start_color()?;

    let colors1 = Colors::new(Color::Light(BaseColor::Red), Color::Dark(BaseColor::Black));
    let colors2 = Colors::new(Color::Light(BaseColor::Yellow), Color::Dark(BaseColor::Black));

    let color_pair0 = ColorPair::default();
    let color_pair1 = ColorPair::new(1, colors1)?;
    let color_pair2 = ColorPair::new(2, colors2)?;

    let mut attrs = Attribute::Normal | color_pair0;

    attr_set(attrs, color_pair0)?;
    addstr("Using modern attribute setting...\n\n")?;

    attr_set(attrs, color_pair1)?;
    addstr("I am Mr. Red!\n")?;
    attr_set(attrs, color_pair2)?;
    addstr("I am Mr. Yellow!\n")?;
    attrs.set_bold(true);
    attr_set(attrs, color_pair1)?;
    addstr("I'm feeling bold!\n")?;
    attr_set(attrs, color_pair2)?;
    addstr("Me too!\n")?;
    refresh()?;

    attrs = Attribute::Normal | color_pair0;
    attrset(attrs)?;
    addstr("\nUsing legacy attribute setting...\n\n")?;

    attrs = attrs | color_pair1;
    attron(attrs)?;
    addstr("I am Mr. Red!\n")?;
    attrs = attrs | color_pair2;
    attron(attrs)?;
    addstr("I am Mr. Yellow!\n")?;
    attrs = attrs | Attribute::Bold | color_pair1;
    attron(attrs)?;
    addstr("I'm feeling bold!\n")?;
    attrs = attrs | Attribute::Bold | color_pair2;
    attron(attrs)?;
    addstr("Me too!\n")?;

    refresh()?;

    getch()?;

    endwin()?;

    Ok(())
}
