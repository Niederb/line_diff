use line_diff::{compare_lines, Config};

use std::error;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    println!("Example without only ';' as separator");
    let config = Config::from_lines(false, false, vec![' '], "Hello;Wor ld", "Hello;Wor ld");
    println!("{:?}", config);
    compare_lines(config)?;

    println!("Example with ';' and ' ' as separators");
    let config = Config::from_lines(false, false, vec![' ', ';'], "Hello;Wor ld", "Hello;Wor ld");
    println!("{:?}", config);
    compare_lines(config)
}
