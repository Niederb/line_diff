use line_diff::{compare_lines, Config};

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example without sorting");
    let config = Config::from_lines(false, false, vec![' '], "Hello World", "World Hello");
    println!("{:?}", config);
    compare_lines(config)?;

    println!("Example with sorting");
    let config = Config::from_lines(true, false, vec![' '], "Hello World", "World Hello");
    println!("{:?}", config);
    compare_lines(config)
}
