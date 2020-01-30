#![forbid(unsafe_code)]

use std::error;

use line_diff::execute;
use line_diff::Config;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let config = Config::from_cmd_args();
    println!("{:#?}", config);
    execute(config)
}
