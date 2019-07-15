use clap::{App, Arg, SubCommand};
use flate2::read::GzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use regex::Regex;
use std::{error::Error, path::PathBuf};
use tar::Archive;

#[derive(Debug)]
enum Host {
    Github,
    Gitlab,
    BitBucket,
}
#[derive(Debug)]
struct Repo {
    host: Host,
    project: String,
    owner: String,
}

fn main() {
    let matches = App::new("degit-rs")
        .version("1.0")
        .author("Vlad Pănăzan <brgdvz@gmail.com>")
        .about("Download the contents of a git repository without cloning it.")
        .arg(
            Arg::with_name("src")
                .help("the source repo ypu want to download")
                .required(true)
                .index(1)
                .validator(validate_src),
        )
        .arg(
            Arg::with_name("dest")
                .help("download location")
                .required(false)
                .index(2),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let dest = matches
        .value_of("dest")
        .map_or(std::env::current_dir().unwrap(), |x| {
            PathBuf::from(x).canonicalize().unwrap()
        });

    let src = matches.value_of("src").unwrap();
    println!("Using source file: {}", src);

    println!("Value for dest: {}", dest.display());
    let repo = parse(src);

    println!("{:#?}", repo);

    let repo = repo.unwrap();
    println!("{:#?}", repo);

    let fname = dest.join(&repo.project);
    download(repo,fname);

}
fn download(repo: Repo, dest: PathBuf)->Result<(), Box<dyn Error>>{
    let url = match repo.host {
        Host::Github => format!(
            "https://github.com/{}/{}/archive/master.tar.gz",
            repo.owner, repo.project
        ),
        Host::Gitlab => format!(
            "https://gitlab.com/{}/{}/repository/archive.tar.gz",
            repo.owner, repo.project
        ),
        Host::BitBucket => "sal".to_string(),
    };
    println!("{}", url);
    // let mut resp = reqwest::get(&url).expect("request failed");
    let client = reqwest::Client::new();

    let request = client.get(&url).send().unwrap();
    let total_size = request.content_length();

    let pb = match total_size {
        Some(x) => {
            let p = ProgressBar::new(x);
            p.set_style(ProgressStyle::default_bar()
                     .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                     .progress_chars("#>-"));
            p
        }
        None => ProgressBar::new_spinner(),
    };

    println!("{:#?}", request.content_length());

    // let mut file = std::fs::File::create(fname).unwrap();
    // std::io::copy(&mut pb.wrap_read(request), &mut file).expect("failed to copy content");
    let tar = GzDecoder::new(pb.wrap_read(request));
    let mut archive = Archive::new(tar);
    archive
            .entries()?
            .filter_map(|e| e.ok())
            .map(|mut entry| -> Result<PathBuf, Box<dyn Error>> {
                let path = entry.path()?;
                let path = path.strip_prefix(path.components().next().unwrap())?.to_owned();
                entry.unpack(dest.join( &path ))?;
                Ok(path)
            })
            .filter_map(|e| e.ok())
            .for_each(|x| println!("> {}", x.display()));

    // archive.unpack(dest).unwrap();
    println!("Hello, world! {}", url);
    Ok(())
}
fn parse(src: &str) -> Result<Repo, Box<dyn Error>> {
    let repo_match = Regex::new(
        r"(?x)
                                (?P<host>(git@|https://)([\w\.@]+)(/|:))
                                (?P<owner>[\w,\-,_]+)
                                /
                                (?P<repo>[\w,\-,_]+)
                                (.git)?/?
                                ",
    )
    .unwrap();
    if repo_match.is_match(src) {
        let caps = repo_match.captures(src).unwrap();
        let host = caps.name("host").unwrap().as_str();
        let hosten;
        // println!("{:#?}", Err("Git provider not supported."));
        if host.contains("github") {
            hosten = Host::Github;
        } else if host.contains("gitlab") {
            hosten = Host::Gitlab;
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

    Err("Could not parse repository")?
}

fn validate_src(src: String) -> Result<(), String> {
    parse(&src).map(|_| ()).map_err(|x| x.to_string())
}
