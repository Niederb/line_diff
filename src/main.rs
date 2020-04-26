#![forbid(unsafe_code)]

use std::error;

use line_diff::compare_lines;
use line_diff::Config;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> Result<()> {
    let config = Config::from_cmd_args();

    //println!("{:#?}", config);
    compare_lines(config)
}
