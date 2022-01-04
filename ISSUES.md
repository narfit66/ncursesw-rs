## Environment

For some reason and i don't understand why at the moment you need to set the environment variable 'TERMINFO' to point to a valid terminfo database such as '/usr/share/terminfo'. You can do this by editing ~/.bashrc and adding the line 'export TERMINFO=/usr/share/terminfo' so every time you run a shell it is defined.
