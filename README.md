Vim plugin for seamless i3/vim navigation
=========================================

Allows i3 direction keys to control vim splits and i3 windows seamlessly

The `i3-vim-focus` folder has a Rust program that should be installed on the
user's path. It can be build with `cargo build --release`.

No changes are necessary to the .vimrc.

Every instance of vim must be launched with a `--servername` argument that
starts with `VIM:` and must be unique. One way to do achieve this is by using an
alias for vim, like so:

```bash
alias vim='/usr/local/bin/vim --servername "VIM:$RANDOM"'
```

The i3 config needs to be updated with the following bindings.

```
bindsym --release $mod+h exec --no-startup-id "i3-vim-focus left"
bindsym --release $mod+j exec --no-startup-id "i3-vim-focus down"
bindsym --release $mod+k exec --no-startup-id "i3-vim-focus up"
bindsym --release $mod+l exec --no-startup-id "i3-vim-focus right"
```

Finally, this project needs to be installed as a vim plugin using
something like pathogen.
