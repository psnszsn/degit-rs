# degit-rs

degit-rs is a rust rewrite of [degit](https://github.com/Rich-Harris/degit/). It downloads copies of git repositories from the internet, without the need of `git clone`. This is faster, since it does not download the `.git` folder (which contains all the git history) and allows you to initialize a new repository afterwards. It is useful for downloading project boilerplate templates.

## Instalation
```
cargo install degit
```
### Advantages over original degit
* does not require nodejs
* does not create files in your home directory
* supports hosted gitlab instances
