# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.5.0] - ????.??.?? [BC]
- NCurses form module implemented as `ncursesw::form`.
- `ncursesw::menu_request_by_name()` now returns a `Result<Option<ncursesw::menu::MenuRequest>, ncursesw::NCurseswMenuError>` instead of `Result<bool, ncursesw::NCurseswMenuError>`. [BC]
- `ncursesw::menu::menu_request_name()` now takes a `ncursesw::menu::MenuRequest` instead of `i32`. [BC]
- `ncursesw::menu::set_menu_pad()` now takes a `char` instead of `i32`. [BC]
- `ncursesw::menu::menu_driver()` now returns a `Result<Option<ncursesw::menu::MenuRequest>, ncursesw::NCurseswMenuError>` instead of `Result<Option<i32>, ncursesw::NCurseswMenuError>`. [BC]
- `bindgen` will now only create bindings against version 6.1 and above of the NCurses library.
- `ncursesw::newterm()` implemented (was calling `unimplemented!()` and signature now takes `O: std::os::unix::io::AsRawFd + std::io::Write` and `I: std::os::unix::io::AsRawFd + std::io::Write` instead of `shims::bindings::FILE` for both. [BC]
- `ncursesw::{scr_dump,scr_init,scr_restore,scr_set}` functions implemented (all where calling `unimplemented!()`, signatures now take `&std::path::Path` instead of `&str` for all. [BC]
- `ncursesw::getwin()` now takes `I: std::os::unix::io::AsRawFd + std::io::Read` instead of `&std::path::Path`. [BC]
- `ncursesw::putwin()` now takes `O: std::os::unix::io::AsRawFd + std::io::Write` instead of `&std::path::Path`. [BC]
- `ncursesw::shims::ncurses::wunctrl()` now returns `Option<*mut wchar_t>` instead of `*mut wchar_t`. [BC]
- Added missing NCurses `_sp` (screen) functions.
- `ncursesw::normal::alloc_pair()` now returns a `Result<ncursesw::normal::ColorPair, ncursesw::NCurseswError>` instead of `ncursesw::normal::ColorPair`. [BC]
- `ncursesw::normal::find_pair()` now returns a `Result<Option<ncursesw::normal::ColorPair>, ncursesw::NCurseswError>` instead of `Option<ncursesw::normal::ColorPair>`. [BC]
- `ncursesw::free_pair()` signature now takes  `i32: From<T>` instead of `i32: From<P>`. [BC]
- Added `ncursesw::{normal,extend}::Attribute::Default` trait which returns `ncursesw::{normal,extend}::Attribute::Normal`.
- Added `ncursesw::{normal,extend}::Color::Default` trait which returns `ncursesw::{normal,extend}::Color::TerminalDefault`.
- Added `ncursesw::{normal,extend}::Colors::Default` trait which returns `ncursesw::{normal,extend}::Colors { foreground: ncursesw::{normal,extend}::Color::TerminalDefault, background: ncursesw::{normal,extend}::Color::TerminalDefault }`.
- Added `ncursesw::{normal,extend}::AttributesColorPair::Default` trait which returns `ncursesw::AttributesColorPair { attributes: ncursesw::{normal,extend}::Attributes::Normal, color_pair: ncursesw::{normal,extend}::ColorPair::default() }`.
- Removed `ncursesw::mouse::set_mouseinterval()`, replaced by `ncursesw::mouse::mouseinterval(std::time::Duration)`. [BC]
- `ncursesw::mouse::mouseinterval()` signature has changed from `mouseinterval(delay: std::time::Duration) -> Result<(), ncursesw::NCurseswMouseError>` to `mouseinterval(delay: Option<std::time::Duration>) -> Result<std::time::Duration, ncursesw::NCurseswMouseError>`. [BC]

## [0.4.0] - 2019-12-09 [BC]
- NCurses menu module implemented as `ncursesw::menu`.
- Minor API changes. [BC]
- NCurses panels module functions are now correctly linked to.
- Depreciated NCurses color type functions in favor of encapsulated color type structures.

## [0.3.2] - 2019-11-02
- NCurses mouse functionality implemented in `ncursesw::mouse`.

## [0.3.1] - 2019-10-25
- ...

## [0.3.0] - 2019-10-15 [BC]
- ...

## [0.2.0] - 2019-10-14 [BC]
- ...

## [0.1.6] - 2019-09-27
- ...

## [0.1.5] - 2019-07-13
- ...

## [0.1.4] - 2019-07-09
- ...

## [0.1.3] - 2019-07-06
- ...

## [0.1.2] - 2019-07-04
- ...

## [0.1.1] - 2019-07-01
- ...

## [0.1.0] - 2019-07-01
- Initial release.
