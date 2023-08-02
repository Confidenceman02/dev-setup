use directories;
use std::path::Path;

fn main() {
    let base_dirs = directories::BaseDirs::new().unwrap();

    // All required paths
    let paths: Vec<&Path> = vec![
        Path::new(".config/alacritty"),
        Path::new(".config/nvim"),
        Path::new(".config/nvim/lua"),
    ];

    for p in paths {
        let dir_exists = base_dirs.home_dir().join(p).exists();
        println!("is directory: {:?}", dir_exists);
    }
}
