use line_diff::{execute, Config};

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example without lowercase");
    let config = Config::from_lines(false, false, vec![' '], "Hello World", "Hello world");
    println!("{:?}", config);
    execute(config)?;

    println!("Example with sorting");
    let config = Config::from_lines(false, true, vec![' '], "Hello World", "Hello world");
    println!("{:?}", config);
    execute(config)
}
