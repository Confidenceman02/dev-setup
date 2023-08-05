use directories::{self, BaseDirs};
use paris::Logger;
use std::path::Path;
use std::{fs, vec};

#[derive(Debug, Copy, Clone)]
enum Directory<'a> {
    Alacritty(&'a Path),
    Nvim(&'a Path),
    Lua(&'a Path),
}

impl Directory<'_> {
    fn to_path(&self) -> &Path {
        match self {
            Directory::Alacritty(p) => p,
            Directory::Nvim(p) => p,
            Directory::Lua(p) => p,
        }
    }
}

fn main() {
    let base_dirs = directories::BaseDirs::new().unwrap();

    // All required paths
    let paths: Vec<Directory> = vec![
        Directory::Alacritty(Path::new(".config/alacritty")),
        Directory::Nvim(Path::new(".config/nvim")),
        Directory::Lua(Path::new(".config/nvim/lua")),
    ];

    let mut log1: Logger = Logger::new();
    let validated_dirs = check_dirs(paths, base_dirs, &mut log1);

    let mut log2: Logger = Logger::new();
    create_dirs(validated_dirs, &mut log2);
}

fn create_dirs<'a>(
    validated_dirs: Vec<Result<Directory<'a>, Directory<'a>>>,
    log: &'a mut Logger,
) -> Vec<Directory<'a>> {
    log.log("<yellow>Creating directories</>");
    let mut created: Vec<Directory> = vec![];
    for dir in validated_dirs {
        match dir {
            Ok(p) => {
                log.log(p.to_path().to_str().unwrap().to_owned() + " <yellow>SKIPPED</>");
                created.push(p);
                Ok(())
            }
            Err(p1) => {
                log.log(p1.to_path().to_str().unwrap().to_owned() + " <green>CREATED</>");
                created.push(p1);
                fs::create_dir(p1.to_path().to_str().unwrap().to_owned())
            }
        }
        .ok();
    }
    created
}

fn check_dirs<'a>(
    paths: Vec<Directory<'a>>,
    base_dirs: BaseDirs,
    log: &'a mut Logger,
) -> Vec<Result<Directory<'a>, Directory<'a>>> {
    log.log("<yellow>Checking directories</>");
    let mut checked: Vec<Result<Directory, Directory>> = vec![];
    for dir in paths {
        let dir_exists = base_dirs.home_dir().join(dir.to_path()).exists();

        if dir_exists {
            log.log(dir.to_path().to_str().unwrap().to_owned() + "<green>" + " \u{2713}" + "</>");
            checked.push(Ok(dir));
        } else {
            log.log(dir.to_path().to_str().unwrap().to_owned() + "<red>" + " \u{10102}" + "</>");
            checked.push(Err(dir));
        }
    }
    checked
}
