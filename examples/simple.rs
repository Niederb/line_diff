use line_diff::{execute, Config};

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let config = Config::from_lines("Hello World", "hello World");
    execute(config)
}
