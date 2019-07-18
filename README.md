# degit-rs

degit-rs is a rust rewrite of [degit](https://github.com/Rich-Harris/degit/). It downloads copies of git repositories from the internet, without the need for `git clone`. This is faster, since it does not download the `.git` folder (which contains all the git history) and allows you to initialize a new repository afterwards. It is useful for downloading project boilerplate templates.

[![Crates.io](https://img.shields.io/crates/v/degit.svg)](https://crates.io/crates/degit)
## Instalation
```
cargo install degit
```
## Usage examples
``````
degit https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template my_new_project
degit sveltejs/template my-svelte-project
``````
## Advantages over original degit
* does not require nodejs
* does not create files in your home directory
* supports hosted gitlab instances
* progress bar

## Todo
* specify a tag, branch or commit 
