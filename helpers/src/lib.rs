pub mod file {
    use std::fs::File;
    use std::io::{BufReader, BufRead};
    use std::path::PathBuf;

    pub fn parse_data<T: std::str::FromStr>(path: PathBuf) -> impl Iterator<Item = T>{
        let file = File::open(&path).expect("Can't read the file");
        let reader = BufReader::new(file);
        reader.lines().map(|l| l.unwrap().parse::<T>().ok().unwrap())
    }
 }

pub mod args {
    use std::env;
    use std::path::{Path, PathBuf};
    pub fn path_from_args() -> Result<PathBuf, &'static str> {
        let args: Vec<String> = env::args().collect();
        if args.len() > 1 {
            let path = Path::new(&args[1]);
            if !path.is_file() {
                return Err("Path doesn't point to a file")
            }
            return Ok(path.to_path_buf())
        }
        Err("You have to provide a file path")
    }
}