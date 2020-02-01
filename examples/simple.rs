use line_diff::{execute, Config};

use std::error;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let config = Config::from_lines(false, false, vec![' '], "Hello World", "hello World");
    execute(config);

    let config = Config::from_file(false, false, vec![' '], PathBuf::from("examples/l1.txt"));
    execute(config)
}
