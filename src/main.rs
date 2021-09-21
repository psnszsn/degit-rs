use clap::{crate_version, App, Arg};
use std::{path::PathBuf};

fn main() {
    let matches = App::new("degit-rs")
        .version(crate_version!())
        .author("Vlad Pănăzan <brgdvz@gmail.com>")
        .about("Download the contents of a git repository without cloning it.")
        .arg(
            Arg::with_name("src")
                .help("the source repo you want to download")
                .long_help(
"The repository you want to download. This can be either the full url or a shortened form:

user/repo
github:user/repo
git@github.com:user/repo
https://github.com/user/repo

gitlab:user/repo
git@gitlab.com:user/repo
https://gitlab.com/user/repo

bitbucket:user/repo
git@bitbucket.org:user/repo
https://bitbucket.org/user/repo

")

                .required(true)
                .index(1)
                .validator(degit::validate_src),
        )
        .arg(
            Arg::with_name("dest")
                .help("download location")
                .long_help("The destination directory. This is where the contents of the repository will be downloaded.")
                .required(false)
                .index(2)
                .validator(degit::validate_dest)
                .default_value("."),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let src = matches.value_of("src").unwrap();

    let dest = PathBuf::from(matches
        .value_of("dest").unwrap());

    degit::degit(src, dest);
}
