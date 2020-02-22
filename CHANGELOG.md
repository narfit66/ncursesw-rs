# Changelog

All breaking changes are marked with [BC] and potentially require API consumer changes after updating to the respective version.

## [0.6.0] - ????.??.??
- Upgraded source code to rust 2018 edition.
- Changed signature of `getsyx() -> Result<Origin, NCurseswError>` to `getsyx() -> Result<Option<Origin>, NCurseswError>` to return a `None` instead of `Origin { y: -1, x: -1 }`. [BC]
- Changed signature of `setsyx(origin: Origin) -> Result<(), NCurseswError>` to `setsyx(origin: Option<Origin>) -> Result<(), NCurseswError>`. [BC]
- Changed signature of `intrflush()` and `intrflush_sp()` to ignore `handle/window` parameter as this is ignored in the NCurses library. [BC]
- Removed `SoftLabelType::FourFourIndex` enum variant and added `SoftLabelType::{FourFourFour, FourFourFourIndex}` enum variants. [BC]
- Changed `slk_attr()` to return `normal::Attributes` instead of `attr_t`. [BC]
- `shims::ncurses::intrflush_sp()` nolonger does an assertion on a null `win` parameter as NCurses documentation indicates that parameter is not required.
- Removed `attr_get_sp()`, `getcchar_sp()` and `wattr_get_sp()` which where non-NCurses function and specific to this crate. [BC]
- Added `fn screen(&self) -> Option<SCREEN>` to `ColorsType` trait. [BC]
- Added `fn screen(&self) -> Option<SCREEN>` to `ColorPairType` trait. [BC]
- Removed `fn new(_: C, _: C) -> Self` from `ColorsType` trait. [BC]
- Removed `AttributesGeneric` and `AttributesColorPairType` traits. [BC]
- Changed enum variant `NCursesColorType::Extended` to `NCursesColorType::Extend`. [BC]
- Changed enum variant `AttributesColorPairSet::Exteneded` to `AttributesColorPairSet::Extend`. [BC]
- Added `AttributesColorPairSet::{unwrap_as_normal, unwrap_as_extend}` methods to unwrap as specific `AttributesColorPairSet` enum variant or panic.
- Rewrite of `AttributesType` trait to only have prototypes of `fn screen(&self) -> Option<SCREEN>` and `fn as_attr_t(&self) -> attr_t`. [BC]
- Changed `Attributes` set type methods to return `Self` and be non-mutating so they can be chained together i.e. `attrs = attrs.set_bold(true).set_blink(true);` as well as `attrs.set_bold(true);`. [BC]
- Changed `form::FormOptions` set type methods to return `Self` and be non-mutating so they can be chained together i.e. `attrs = attrs.set_newline_overload(true).set_backspace_overload(true);` as well as `attrs.set_newline_overload(true);`. [BC]
- Changed `form::FieldOptions` set type methods to return `Self` and be non-mutating so they can be chained together i.e. `attrs = attrs.set_edit(true).set_auto_skip(true);` as well as `attrs.set_edit(true);`. [BC]
- Changed `menu::MenuOptions` set type methods to return `Self` and be non-mutating so they can be chained together i.e. `attrs = attrs.set_show_description(true).set_mouse_menu(true);` as well as `attrs.set_show_description(true);`. [BC]
- Changed `menu::MenuOptions` set type methods to return `Self` and be non-mutating so they can be chained together (at the moment there is only one option). [BC]
- Changed the following functions in `form` and `shims::nform:nform` to have there signatures changed to replace the `FORM` parameter to `Option<FORM>` : `current_field()`, `field_count()`, `field_init()`, `field_term()`, `form_fields()`, `form_init()`, `form_opts()`, `form_opts_off()`, `form_opts_on()`, `form_page()`, `form_sub()`, `form_term()`, `form_userptr()`, `form_win()`, `set_field_init()`, `set_field_term()`, `set_form_init()`, `set_form_opts()`, `set_form_term()`, `set_form_userptr()`. [BC]
- Changed the following functions in `form` and `shims::nform:nform` to have there signatures changed to replace the `FIELD` parameter to `Option<FIELD>` : `field_arg()`, `field_back()`, `field_fore()`, `field_just()`, `field_opts()`, `field_opts_off()`, `field_opts_on()`, `field_pad()`, `field_status()`, `field_type()`, `field_userptr()`, `new_page()`, `set_field_back()`, `set_field_fore()`, `set_field_just()`, `set_field_opts()`, `set_field_pad()`, `set_field_status()`, `set_field_type()`, `set_field_userptr()`, `set_new_page()`. [BC]
- Changed the following functions in `menu` and `shims::nmenu` to have there signatures changed to replace the `MENU` parameter to `Option<MENU>` : `item_init()`, `item_term()`, `menu_back()`, `menu_fore()`, `menu_format()`, `menu_grey()`, `menu_init()`, `menu_mark()`, `menu_opts()`, `menu_opts_off()`, `menu_opts_on()`, `menu_pad()`, `menu_spacing()`, `menu_sub()`, `menu_term()`, `menu_userptr()`, `menu_win()`, `set_item_init()`, `set_item_term()`, `set_menu_back()`, `set_menu_fore()`, `set_menu_grey()`, `set_menu_init()`, `set_menu_mark()`, `set_menu_opts()`, `set_menu_pad()`, `set_menu_spacing()`, `set_menu_term()`, `set_menu_userptr()`. [BC]
- Changed the following functions in `menu` and `shims::nmenu` to have there signatures changed to replace the `ITEM` parameter to `Option<ITEM>` : `item_opts()`, `item_opts_off()`, `item_opts_on()`, `item_userptr()`, `set_item_opts()`, `set_item_userptr()`. [BC]
- Have removed examples "showing" how to use individual functions.

The way that colors are defined has been changed as of this release to cater for screen functionality. The `BaseColor` enum has been replaced by the `ColorPalette` enum which defines the basic colors available (which can be considered the dark colors and were originally wrapped in the `BaseColor::Dark()` enum) and the extended light colors (originally wrapped in the `BaseColor::Light()` enum). [BC]

When the client code is using screens, functions that return `Attributes` and `ColorPair` types such as `attr_get()`, `wattr_get()` and `getcchar()` will always set the screen attribute of what they are returning as a `None`, it is upto the clinet code to rectify this by calling `Attributes::set_screen()` and/or `ColorPair::set_screen()`.

Although deprecated it should be noted that the following functions have changed their signatures and should be considered as breaking changes:
- `COLOR_PAIR(color_pair: normal::ColorPair) -> attr_t` has become `COLOR_PAIR(color_pair: i32) -> attr_t`
- `PAIR_NUMBER(attrs: normal::Attributes) -> normal::ColorPair` has become `PAIR_NUMBER(attrs: attr_t) -> short_t`
- `color_content(color_number: normal::Color) -> Result<normal::RGB, NCurseswError>` has become `color_content(color_number: short_t) -> Result<normal::RGB, NCurseswError>`
- `extended_color_content(color_number: extend::Color) -> Result<extend::RGB, NCurseswError>` has become `extended_color_content(color_number: i32) -> Result<extend::RGB, NCurseswError>`
- `extended_pair_content(color_pair: extend::ColorPair) -> Result<extend::Colors, NCurseswError>` has become `extended_pair_content(color_pair: i32) -> Result<extend::Colors, NCurseswError>`
- `init_color(color_number: short_t, rgb: normal::RGB) -> Result<normal::Color, NCurseswError>` has become `init_color(color_number: short_t, rgb: normal::RGB) -> Result<(), NCurseswError>`
- `init_extended_color(color_number: i32, rgb: extend::RGB) -> Result<extend::Color, NCurseswError>` has become `init_extended_color(color_number: i32, rgb: extend::RGB) -> Result<(), NCurseswError>`
- `pair_content(color_pair: normal::ColorPair) -> Result<normal::Colors, NCurseswError>` has become `pair_content(color_pair: short_t) -> Result<normal::Colors, NCurseswError>`
- `color_content_sp(screen: SCREEN, color_number: normal::Color) -> Result<normal::RGB, NCurseswError>` has become `color_content_sp(screen: SCREEN, color_number: short_t) -> Result<normal::RGB, NCurseswError>`
- `extended_color_content_sp(screen: SCREEN, color_number: extend::Color) -> Result<extend::RGB, NCurseswError>` has become `extended_color_content_sp(screen: SCREEN, color_number: i32) -> Result<extend::RGB, NCurseswError>`
- `extended_pair_content_sp(screen: SCREEN, color_pair: extend::ColorPair) -> Result<extend::Colors, NCurseswError>` has become `extended_pair_content_sp(screen: SCREEN, color_pair: i32) -> Result<extend::Colors, NCurseswError>`
- `init_color_sp(screen: SCREEN, color_number: short_t, rgb: normal::RGB) -> Result<normal::Color, NCurseswError>` has become `init_color_sp(screen: SCREEN, color_number: short_t, rgb: normal::RGB) -> Result<(), NCurseswError>`
- `init_extended_color_sp(screen: SCREEN, color_number: i32, rgb: extend::RGB) -> Result<extend::Color, NCurseswError>` has become `init_extended_color_sp(screen: SCREEN, color_number: i32, rgb: extend::RGB) -> Result<(), NCurseswError>`
- `pair_content_sp(screen: SCREEN, color_pair: normal::ColorPair) -> Result<normal::Colors, NCurseswError>` has become `pair_content_sp(screen: SCREEN, color_pair: short_t) -> Result<normal::Colors, NCurseswError>`

## [0.5.1] - 2020.01.30
- Fix so that crate compiles on `docs.rs` for documentation.

## [0.5.0] - 2020.01.29 [BC]
- `bindgen` will now only create bindings against version 6.1 and above of the NCurses library.
- NCurses form module implemented as `form`.
- Added missing NCurses `_sp` (screen) functions.
- Added missing `is_subwin()`, `getsyx()` and `setyx()` routines (`{get,set}syx()` are not implemented in the `ncursesw::shims::ncurses` layer).
- Added `{normal,extend}::Attribute::Default` trait which returns `{normal,extend}::Attribute::Normal`.
- Added `{normal,extend}::Color::Default` trait which returns `{normal,extend}::Color::TerminalDefault`.
- Added `{normal,extend}::Colors::Default` trait which returns `{normal,extend}::Colors { foreground: {normal,extend}::Color::TerminalDefault, background: {normal,extend}::Color::TerminalDefault }`.
- Added `{normal,extend}::AttributesColorPair::Default` trait which returns `AttributesColorPair { attributes: {normal,extend}::Attributes::Normal, color_pair: {normal,extend}::ColorPair::default() }`.
- Removed `setlocale()` and `LcCategory` [BC].
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
