# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.5.0] - ????.??.?? [BC]
- NCurses form module implemented as `ncursesw::form`.
- `ncursesw::menu::menu_request_name()` now takes a `MenuRequest` instead of `i32`. [BC]
- `ncursesw::menu::set_menu_pad()` now takes a `char` instead of `i32`. [BC]
- `ncursesw::menu::menu_driver()` now returns a `Result<Option<MenuRequest>, NCurseswMenuError>` instead of `Result<Option<i32>, NCurseswMenuError>`. [BC]
- `bindgen` will now only create bindings against version 6.1 and above of the NCurses library.

## [0.4.0] - 2019-12-09 [BC]
- NCurses menu module implemented as `ncursesw::menu`.
- Minor API changes. [BC]
- NCurses panels module functions are now correctly linked to.

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
