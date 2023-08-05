use directories::{self, BaseDirs};
use paris::Logger;
use std::path::Path;

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
    let mut log: Logger = Logger::new();
    let base_dirs = directories::BaseDirs::new().unwrap();

    // All required paths
    let paths: Vec<Directory> = vec![
        Directory::Alacritty(Path::new(".config/alacritty")),
        Directory::Nvim(Path::new(".config/nvim")),
        Directory::Lua(Path::new(".config/nvim/lua")),
    ];

    check_dirs(paths, base_dirs, &mut log);
    // TODO Create directories that are missing
}


fn check_dirs<'a>(
    paths: Vec<Directory<'a>>,
    base_dirs: BaseDirs,
    log: &'a mut Logger,
) -> Vec<Result<Directory<'a>, String>> {
    log.log("<yellow>Checking directories</>");
    let mut checked: Vec<Result<Directory, String>> = vec![];
    for dir in paths {
        let dir_exists = base_dirs.home_dir().join(dir.to_path()).exists();

        if dir_exists {
            log.log(dir.to_path().to_str().unwrap().to_owned() + "<green>" + " \u{2713}" + "</>");
            checked.push(Ok(dir));
        } else {
            log.log(dir.to_path().to_str().unwrap().to_owned() + "<red>" + " \u{10102}" + "</>");
            checked.push(Err("The directory ".to_owned()
                + dir.to_path().to_str().unwrap()
                + " doesn't exist"));
        }
    }
    checked
}
