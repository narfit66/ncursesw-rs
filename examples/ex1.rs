extern crate ncursesw;

use std::thread::sleep;
use std::time::Duration;
use std::process::exit;

use ncursesw::*;

fn main() {
    println!("curses version:  {:?}", curses_version());
    println!("ncurses version: {:?}", ncurses_version());

    sleep(Duration::from_secs(10));

    match doit() {
        Ok(_)  => {
            let _ = endwin();
            exit(0)
        },
        Err(e) => {
            let _ = endwin();
            println!("error: {}", e);
            exit(1);
        }
    }
}

fn doit() -> Result<(), NCurseswError> {
    let win = initscr()?;

    raw()?;
    noecho()?;
    notimeout(win, true)?;

    curs_set(CursorType::Invisible)?;

    //r#box(win, &char::from(0x7c), &char::from(0x2d))?;

    let mut origin = Origin { y: 1, x: 1 };

    let test_str = "window has been created!!! this was with mvwaddstr()";
    let test_widestr = WideString::from_str("...and this is with mvwaddwstr()");

    mvwaddstr(win, origin, &test_str)?;

    origin.y += 1;
    mvwaddwstr(win, origin, &test_widestr)?;

    origin.y += 1;
    mvwaddstr(win, origin, "input upto 10 chracters: ")?;

    wrefresh(win)?;

    curs_set(CursorType::Visible)?;
    echo()?;

    let str = wgetnstr(win, 10)?;

    origin.y += 1;
    mvwaddstr(win, origin, &format!("got: {}", str))?;

    origin.y += 1;
    mvwaddstr(win, origin, "get a wide string upto 10 characters: ")?;

    wrefresh(win)?;

    let wstr = wgetn_wstr(win, 10)?;

    origin.y += 1;
    mvwaddwstr(win, origin, &wstr)?;

    wrefresh(win)?;

    sleep(Duration::from_secs(10));

    Ok(())
}
