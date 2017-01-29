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
