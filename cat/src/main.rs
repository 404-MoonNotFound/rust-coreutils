use std::env;
use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

fn main() {
    let mut path;

    let file_paths = env::args();

    for (index, path_string) in file_paths.enumerate() {
        if index == 0 {
            continue;
        }
        path = Path::new(&path_string);

        match cat(path) {
            Ok(s) => {
                println!("\n{}: \n{}", path.file_name().unwrap().to_str().unwrap(), s);
            }
            Err(e) => println!("\nError: {}", e),
        }
    }
}

fn cat(path: &Path) -> io::Result<String> {
    let mut f = File::open(path)?;
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
