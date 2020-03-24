use line_diff::{compare_lines, Config};

use std::error;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example with using a single file containing two lines");
    let config = Config::from_file(
        false,
        false,
        vec![' '],
        PathBuf::from("examples/long-lines.txt"),
    );
    println!("{:?}", config);
    compare_lines(config)
}
