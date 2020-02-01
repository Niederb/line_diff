use line_diff::{execute, Config};

use std::error;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example with two lines");
    let config = Config::from_lines(false, false, vec![' '], "Hello World", "hello World");
    println!("{:?}", config);
    execute(config)?;

    println!("Example with using a single file containing two lines");
    let config = Config::from_file(false, false, vec![' '], PathBuf::from("examples/l1.txt"));
    println!("{:?}", config);
    execute(config)
}
