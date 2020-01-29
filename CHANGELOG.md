# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.5.0] - 2020.01.29 [BC]
- `bindgen` will now only create bindings against version 6.1 and above of the NCurses library.
- NCurses form module implemented as `form`.
- Added missing NCurses `_sp` (screen) functions.
- Added missing `is_subwin()`, `getsyx()` and `setyx()` routines (`{get,set}syx()` are not implemented in the `ncursesw::shims::ncurses` layer).
- Added `{normal,extend}::Attribute::Default` trait which returns `{normal,extend}::Attribute::Normal`.
- Added `{normal,extend}::Color::Default` trait which returns `{normal,extend}::Color::TerminalDefault`.
- Added `{normal,extend}::Colors::Default` trait which returns `{normal,extend}::Colors { foreground: {normal,extend}::Color::TerminalDefault, background: {normal,extend}::Color::TerminalDefault }`.
- Added `{normal,extend}::AttributesColorPair::Default` trait which returns `AttributesColorPair { attributes: {normal,extend}::Attributes::Normal, color_pair: {normal,extend}::ColorPair::default() }`.
- Removed `setlocale()` and `LcCategory`.
- `newterm()` implemented (was calling `unimplemented!()` and signature now takes `O: std::os::unix::io::AsRawFd + std::io::Write` and `I: std::os::unix::io::AsRawFd + std::io::Write` instead of `shims::bindings::FILE` for both. [BC]
- `{scr_dump,scr_init,scr_restore,scr_set}` functions implemented (all where calling `unimplemented!()`, signatures now take `&std::path::Path` instead of `&str` for all. [BC]
- `getwin()` now takes `I: std::os::unix::io::AsRawFd + std::io::Read` instead of `&std::path::Path`. [BC]
- `putwin()` now takes `O: std::os::unix::io::AsRawFd + std::io::Write` instead of `&std::path::Path`. [BC]
- `normal::alloc_pair()` now returns a `Result<normal::ColorPair, NCurseswError>` instead of `normal::ColorPair`. [BC]
- `normal::find_pair()` now returns a `Result<Option<normal::ColorPair>, NCurseswError>` instead of `Option<normal::ColorPair>`. [BC]
- `free_pair()` signature now takes  `i32: From<T>` instead of `i32: From<P>`. [BC]
- `shims::ncurses::wunctrl()` now returns `Option<*mut wchar_t>` instead of `*mut wchar_t`. [BC]
- `menu_request_by_name()` now returns a `Result<Option<menu::MenuRequest>, NCurseswMenuError>` instead of `Result<bool, NCurseswMenuError>`. [BC]
- `menu::menu_request_name()` now takes a `menu::MenuRequest` instead of `i32`. [BC]
- `menu::set_menu_pad()` now takes a `char` instead of `i32`. [BC]
- `menu::menu_driver()` now returns a `Result<Option<menu::MenuRequest>, NCurseswMenuError>` instead of `Result<Option<i32>, NCurseswMenuError>`. [BC]
- `mouse::mouseinterval()` signature has changed from `mouseinterval(delay: std::time::Duration) -> Result<(), mouse::NCurseswMouseError>` to `mouseinterval(delay: Option<std::time::Duration>) -> Result<std::time::Duration, mouse::NCurseswMouseError>`. [BC]
- `mouse::mousemask()` signature changed from `mousemask(newmask: mmask_t, oldmask: Option<*mut mmask_t>) -> Result<mouse::mmask_t, mouse::NCurseswMouseError>` to `mousemask(newmask: mmask_t) -> Result<mouse::mmask_t, mouse::NCurseswMouseError>`. [BC]
- Removed `mouse::set_mouseinterval()` which is replaced by calling `mouse::mouseinterval(Some(std::time::Duration))`. [BC]

## [0.4.0] - 2019-12-09 [BC]
- NCurses menu module implemented as `menu`.
- Minor API changes. [BC]
- NCurses panels module functions are now correctly linked to.
- Depreciated NCurses color type functions in favor of encapsulated color type structures.

## [0.3.2] - 2019-11-02
- NCurses mouse functionality implemented in `mouse`.

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
