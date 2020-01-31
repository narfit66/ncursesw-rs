## TODO's

- Finish documentation!!!
- Write soft labels for the `normal` and `extend` modules.
- Revisit `form` and `menu` modules with regards to set defaults with `Option<FORM>` and `Option<MENU>`, see NCurses documention.
- In next breaking change signature of `getsyx() -> Result<Origin, NCurseswError>` to `getsyx() -> Result<Option<Origin>, NCurseswError>` to return a `None` instead of `Origin { y: -1, x: -1 }`.
- In next breaking change signature of `intrflush()` and `intrflush_sp()` to ignore `handle/window` parameter as this is ignored in the NCurses library.
- Need an example for the `form` module.
- Need an example of screen functionality using stdin and stdout and also xterm sessions (the crate `nix` has a `openpty()` routine).
- Do we need examples of individual routines? I think not!
