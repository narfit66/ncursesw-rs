## TODO's

- Finish documentation!!!
- Revisit `form` and `menu` modules with regards to set defaults with `Option<FORM>` and `Option<MENU>`, see NCurses documention.
- Revisit `normal` and `extend` modules, thinking that `ColorPair` should lose `Copy` and `Clone` traits, we can also change signature of `ColorPair::new()` and `ColorPair::new_sp()` to not habe `pair` parameter and generate this ourselfs, only question is how we then deal with the default color pair, by doing this we would also call `free_pair()` on drop and manage the internal color pair list ourselfs.
- Revisit `normal` and `extend` Color as this does not work correctly with screen (`_sp`) functions.
- In next breaking change signature of `getsyx() -> Result<Origin, NCurseswError>` to `getsyx() -> Result<Option<Origin>, NCurseswError>` to return a `None` instead of `Origin { y: -1, x: -1 }`.
- In next breaking change signature of `intrflush()` and `intrflush_sp()` to ignore `handle/window` parameter as this is ignored in the NCurses library.
- Need an example for the `form` module.
- Need an example of screen functionality using stdin and stdout and also xterm sessions (the crate `nix` has a `openpty()` routine).
- Do we need examples of individual routines? I think not!
