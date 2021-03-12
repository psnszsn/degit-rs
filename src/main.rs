use clap::{crate_version, App, Arg};
use colored::*;
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{error::Error, fmt, path::PathBuf};
use tar::Archive;

#[derive(Debug, PartialEq)]
enum Host {
    Github,
    Gitlab(String),
    BitBucket,
}
#[derive(Debug, PartialEq)]
struct Repo {
    host: Host,
    project: String,
    owner: String,
}

impl fmt::Display for Repo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let owner = self.owner.bold().underline();
        let project = self.project.red();
        let host = match self.host {
            Host::Github => "GitHub".blue(),
            Host::Gitlab(_) => "GitLab".red(),
            Host::BitBucket => "BitBucket".green(),
        };
        write!(f, "{}/{} from {}", owner, project, host)
    }
}

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
                .validator(validate_src),
        )
        .arg(
            Arg::with_name("dest")
                .help("download location")
                .long_help("The destination directory. This is where the contents of the repository will be downloaded.")
                .required(false)
                .index(2)
                .validator(validate_dest)
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

    let repo = parse(src);
    let repo = repo.unwrap();

    let dest = matches
        .value_of("dest")
        .map_or(std::env::current_dir().unwrap().join(&repo.project), |x| {
            PathBuf::from(x)
        });

    match download(repo, dest) {
        Err(x) => println!("{}", x),
        _ => (),
    }
}
fn download(repo: Repo, dest: PathBuf) -> Result<(), Box<dyn Error>> {
    let url = match &repo.host {
        Host::Github => format!(
            "https://github.com/{}/{}/archive/HEAD.tar.gz",
            repo.owner, repo.project
        ),
        Host::Gitlab(x) => format!(
            "https://{}/{}/{}/repository/archive.tar.gz",
            x, repo.owner, repo.project
        ),
        Host::BitBucket => format!(
            "https://bitbucket.org/{}/{}/get/HEAD.zip",
            repo.owner, repo.project
        ),
    };
    // println!("{}", url);
    let client = reqwest::Client::new();

    let request = client.get(&url).send().unwrap();
    match request.status() {
        reqwest::StatusCode::OK => (),
        reqwest::StatusCode::UNAUTHORIZED => {
            Err("Could not find repository.")?;
        }
        s => Err(format!("Received response status: {:?}", s))?,
    };

    let total_size = request.content_length();

    let pb = match total_size {
        Some(x) => {
            let p = ProgressBar::new(x);
            p.set_style(ProgressStyle::default_bar()
                     .template("> {wide_msg}\n{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                     .progress_chars("#>-"));
            p
        }
        None => {
            let p = ProgressBar::new_spinner();
            p
        }
    };

    println!("Downloading {} to {}", repo, dest.display());
    // println!("{:#?}", request.content_length());

    let tar = GzDecoder::new(pb.wrap_read(request));
    let mut archive = Archive::new(tar);
    archive
        .entries()?
        .filter_map(|e| e.ok())
        .map(|mut entry| -> Result<PathBuf, Box<dyn Error>> {
            let path = entry.path()?;
            let path = path
                .strip_prefix(path.components().next().unwrap())?
                .to_owned();
            entry.unpack(dest.join(&path))?;
            Ok(path)
        })
        .filter_map(|e| e.ok())
        .for_each(|x| pb.set_message(&format!("{}", x.display())));

    // archive.unpack(dest).unwrap();
    pb.finish_with_message("Done...");
    Ok(())
}
fn parse(src: &str) -> Result<Repo, Box<dyn Error>> {
    let repo_match = Regex::new(
        r"(?x)
                                (?P<protocol>(git@|https://))
                                (?P<host>([\w\.@]+))
                                (/|:)
                                (?P<owner>[\w,\-,_]+)
                                /
                                (?P<repo>[\w,\-,_]+)
                                (.git)?/?
                                ",
    )
    .unwrap();
    let shortrepo_match = Regex::new(
        r"(?x)
                                (?P<host>(github|gitlab|bitbucket)?)
                                (?P<colon>(:))?
                                (?P<owner>[\w,\-,_]+)
                                /
                                (?P<repo>[\w,\-,_]+)
                                ",
    )
    .unwrap();
    if repo_match.is_match(src) {
        let caps = repo_match.captures(src).unwrap();
        let host = caps.name("host").unwrap().as_str();
        // println!("{}",host);
        let hosten;
        if host.contains("github") {
            hosten = Host::Github;
        } else if host.contains("gitlab") {
            hosten = Host::Gitlab(host.to_string());
        } else if host.contains("bitbucket") {
            hosten = Host::BitBucket;
        } else {
            return Err("Git provider not supported.")?;
        }
        let res = Repo {
            owner: caps.name("owner").unwrap().as_str().to_string(),
            project: caps.name("repo").unwrap().as_str().to_string(),
            host: hosten,
        };
        return Ok(res);
    }
    if shortrepo_match.is_match(src) {
        let caps = shortrepo_match.captures(src).unwrap();
        let host = caps.name("host").unwrap().as_str();
        let colon = caps.name("colon");
        let hosten;
        if let None = colon {
            hosten = Host::Github;
        } else {
            if host.contains("github") {
                hosten = Host::Github;
            } else if host.contains("gitlab") {
                hosten = Host::Gitlab("gitlab.com".to_string());
            } else if host.contains("bitbucket") {
                hosten = Host::BitBucket;
            } else {
                return Err("Git provider not supported.")?;
            }
        }
        let res = Repo {
            owner: caps.name("owner").unwrap().as_str().to_string(),
            project: caps.name("repo").unwrap().as_str().to_string(),
            host: hosten,
        };
        return Ok(res);
    }
    Err("Could not parse repository")?
}

fn validate_src(src: String) -> Result<(), String> {
    parse(&src).map(|_| ()).map_err(|x| x.to_string())
}
fn validate_dest(dest: String) -> Result<(), String> {
    let path = PathBuf::from(dest);
    if path.exists() {
        if path.is_dir() {
            let count = std::fs::read_dir(&path).map_err(|x| x.to_string())?.count();
            if count != 0 {
                Err("Directory is not empty.")?
            }
        } else {
            Err("Destination is not a directory.")?
        }
    }
    let mut realpath = {
        if path.is_relative() {
            let mut realpath = std::fs::canonicalize(std::path::Path::new(".")).unwrap();

            for c in path.components() {
                // println!("component: {:?}", c);
                match c {
                    std::path::Component::ParentDir => {
                        realpath = realpath.parent().unwrap().to_path_buf()
                    }
                    std::path::Component::Normal(c) => realpath.push(c),
                    _ => (),
                }
            }
            realpath
        } else {
            path
        }
    };
    while !realpath.exists(){
        realpath.pop();
    }
    if std::fs::metadata(&realpath).unwrap().permissions().readonly(){
        Err("Directory is read-only.")?
    }
    // println!("realpath: {:?}", realpath);

    Ok(())
}

#[cfg(test)]
mod tests;
