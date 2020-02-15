## TODO's

- Finish documentation!!!
- Revisit `normal` and `extend` modules, thinking that `ColorPair` should drop `Copy` and `Clone` traits, we can also change signature of `ColorPair::new()` and `ColorPair::new_sp()` to not habe `pair` parameter and generate this ourselfs, only question is how we then deal with the default color pair, by doing this we would also call `free_pair()` on drop and manage the internal color pair list ourselfs. Not sure if this should be this crate or `ncursesw`.
- Need an example for the `form` module.
- Need an example of screen functionality using `stdin` and `stdout` and also `xterm` sessions (the crate `nix` has a `openpty()` routine).
