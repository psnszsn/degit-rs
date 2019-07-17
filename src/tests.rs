
    use super::*;

    #[test]
    fn gitlab_test() {
        let repo = Repo {
            host: Host::Gitlab("gitlab.com".to_string()),
            owner: "Rich-Harris".to_string(),
            project: "degit-test-repo".to_string(),
        };

        assert_eq!(
            parse("gitlab:Rich-Harris/degit-test-repo").unwrap(),
            repo
        );
        assert_eq!(
            parse("https://gitlab.com/Rich-Harris/degit-test-repo.git").unwrap(),
            repo
        );
        assert_eq!(
            parse("git@gitlab.com:Rich-Harris/degit-test-repo.git").unwrap(),
            repo
        );
        assert_eq!(
            download(repo, PathBuf::from("/tmp/tests")).unwrap(),
            ()
        );
    }
    #[test]
    fn github_test() {
        let repo = Repo {
            host: Host::Github,
            owner: "sveltejs".to_string(),
            project: "template".to_string(),
        };

        assert_eq!(
            parse("sveltejs/template").unwrap(),
            repo
        );
        assert_eq!(
            parse("github:sveltejs/template").unwrap(),
            repo
        );
        assert_eq!(
            parse("https://github.com/sveltejs/template.git").unwrap(),
            repo
        );
        assert_eq!(
            parse("git@github.com:sveltejs/template.git").unwrap(),
            repo
        );
        assert_eq!(
            download(repo, PathBuf::from("/tmp/tests")).unwrap(),
            ()
        );
    }
  #[test]
    fn gitlab_hosted() {
        let repo = Repo {
            host: Host::Gitlab("gitlab.gnome.org".to_string()),
            owner: "bilelmoussaoui".to_string(),
            project: "gtk-rust-template".to_string(),
        };

        assert_eq!(
            parse("https://gitlab.gnome.org/bilelmoussaoui/gtk-rust-template").unwrap(),
            repo
        );
        assert_eq!(
            parse("git@gitlab.gnome.org:bilelmoussaoui/gtk-rust-template.git").unwrap(),
            repo
        );
        assert_eq!(
            download(repo, PathBuf::from("/tmp/tests")).unwrap(),
            ()
        );
    }
