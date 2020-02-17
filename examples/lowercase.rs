use line_diff::{compare_lines, Config};

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example without lowercase");
    let config = Config::from_lines(false, false, vec![' '], "Hello World", "Hello world");
    println!("{:?}", config);
    compare_lines(config)?;

    println!("Example with sorting");
    let config = Config::from_lines(false, true, vec![' '], "Hello World", "Hello world");
    println!("{:?}", config);
    compare_lines(config)
}
