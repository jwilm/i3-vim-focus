Vim plugin for seamless i3/vim navigation
=========================================

Allows i3 direction keys to control vim splits and i3 windows seamlessly

The `i3-vim-focus` folder has a Rust program that should be installed on the
user's path. It can be build with `cargo build --release`.

The following should be added to your `~/.vimrc`

```viml
map gwl :call Focus('right', 'l')<CR>
map gwh :call Focus('left', 'h')<CR>
map gwk :call Focus('up', 'k')<CR>
map gwj :call Focus('down', 'j')<CR>
```

The i3 config needs to be updated with the following bindings.

```
bindsym $mod+h exec "i3-vim-focus left"
bindsym $mod+j exec "i3-vim-focus down"
bindsym $mod+k exec "i3-vim-focus up"
bindsym $mod+l exec "i3-vim-focus right"
```

Finally, this project needs to be installed as a vim plugin using
something like pathogen.
