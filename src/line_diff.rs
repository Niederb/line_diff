#![forbid(unsafe_code)]

use std::error;
use std::fs::File;
use std::io::prelude::*;
use std::io::{BufRead, BufReader};
use std::path::Path;

use textwrap::{fill, termwidth};

use difference::Difference::{Add, Rem, Same};
use difference::{Changeset, Difference};

use prettytable::{cell, format, row, Table};

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

use std::path::PathBuf;
use structopt::StructOpt;

/// Configuration struct for comparing two lines
#[derive(StructOpt, Debug)]
#[structopt(author, about)]
pub struct Config {
    /// Whether or not the chunks should be sorted before comparing.
    #[structopt(short = "o", long)]
    sort: bool,

    /// Whether or not to convert the chunks to lowercase before comparing
    #[structopt(short, long)]
    lowercase: bool,

    /// Separator for splitting lines. It is possible to define multiple separators.
    /// Newline is always a separator
    #[structopt(short, long, default_value = " ")]
    separators: Vec<char>,

    /// A single file containing two lines. Additional lines will be ignored.
    #[structopt(short, long, parse(from_os_str))]
    file: Option<PathBuf>,

    /// Path to file containing the first line. The complete file will be processed.
    file1: Option<PathBuf>,

    /// Path to file containing the second line. The complete file will be processed.
    file2: Option<PathBuf>,

    /// First line as string
    #[structopt(long)]
    line1: Option<String>,

    /// Second line as string
    #[structopt(long)]
    line2: Option<String>,

    /// File to write the first line after preprocessing to
    #[structopt(long, short = "m")]
    output_file1: Option<PathBuf>,

    /// File to write the second line after preprocessing to
    #[structopt(long, short = "n")]
    output_file2: Option<PathBuf>,
}

impl Config {
    /// Create a config struct by using command line arguments
    pub fn from_cmd_args() -> Config {
        let mut c = Config::from_args();
        c.separators.push('\n');
        c
    }

    /// Create a Config struct that can be used to compare two lines that are given as &str
    /// * `sort` Whether or not to sort chunks before comparing
    /// * `lowercase` Whether or not to convert chunks to lowercase before comparing
    /// * `separators` List of separators to use for splitting lines into chunks
    /// * `l1` The first line
    /// * `l2` The second line
    pub fn from_lines(
        sort: bool,
        lowercase: bool,
        separators: Vec<char>,
        l1: &str,
        l2: &str,
    ) -> Config {
        Config {
            sort,
            lowercase,
            separators,
            file: Option::None,
            file1: Option::None,
            file2: Option::None,
            line1: Option::Some(l1.to_string()),
            line2: Option::Some(l2.to_string()),
            output_file1: Option::None,
            output_file2: Option::None,
        }
    }

    /// Create a Config struct that can be used to compare two lines that are stored in a single file
    /// * `sort` Whether or not to sort chunks before comparing
    /// * `lowercase` Whether or not to convert chunks to lowercase before comparing
    /// * `separators` List of separators to use for splitting lines into chunks
    /// * `filepath` Path to the file that contains the two lines
    pub fn from_file(
        sort: bool,
        lowercase: bool,
        separators: Vec<char>,
        filepath: PathBuf,
    ) -> Config {
        Config {
            sort,
            lowercase,
            separators,
            file: Option::Some(filepath),
            file1: Option::None,
            file2: Option::None,
            line1: Option::None,
            line2: Option::None,
            output_file1: Option::None,
            output_file2: Option::None,
        }
    }
}

struct LineData {
    name: String,
    line: String,
    preprocessed: String,
}

impl LineData {
    fn new(name: &str, line: &str) -> LineData {
        LineData {
            name: name.to_string(),
            line: line.to_string(),
            preprocessed: "".to_string(),
        }
    }

    fn length(&self) -> usize {
        self.line.chars().count()
    }

    fn number_chunks(&self) -> usize {
        self.preprocessed.matches('\n').count() + 1
    }

    fn preprocess_chunks(&mut self, separator: &[char], sort: bool, lowercase: bool) {
        let case_adjusted = if lowercase {
            self.line.to_lowercase()
        } else {
            self.line.to_owned()
        };
        let mut chunks: Vec<&str> = case_adjusted.split(|c| separator.contains(&c)).collect();
        if sort {
            chunks.sort();
        }
        self.preprocessed = chunks.join("\n");
    }
}

fn verify_existing_file(path: &Path) -> Result<()> {
    if !path.exists() {
        Err(format!("Cannot find file1: {}", path.display()).into())
    } else if !path.is_file() {
        Err(format!("Is not a file: {}", path.display()).into())
    } else {
        Ok(())
    }
}

fn get_lines_from_file(path: &Path) -> Result<LineData> {
    verify_existing_file(path)?;

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);

    let mut s = "".to_owned();
    reader.read_to_string(&mut s)?;

    let file_name = if let Some(file_name) = path.file_name() {
        if let Ok(file_name) = file_name.to_os_string().into_string() {
            file_name
        } else {
            "".into()
        }
    } else {
        "".into()
    };
    Ok(LineData::new(&file_name, &s))
}

fn get_two_lines_from_file(path: &Path) -> Result<(LineData, LineData)> {
    verify_existing_file(path)?;

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let mut s1 = "".to_owned();
    let mut s2 = "".to_owned();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            s1 = line.to_owned();
        } else if index == 1 {
            s2 = line.to_owned();
        } else {
            println!("File contains additional lines that will be ignored");
            break;
        }
    }
    Ok((LineData::new("Line 1", &s1), LineData::new("Line 2", &s2)))
}

fn get_line_from_cmd(line_number: i32) -> LineData {
    println!("Please provide line #{}: ", line_number);
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("");
    LineData::new(&format!("Line {}", line_number), &buffer.trim().to_string())
}

fn get_line(line_number: i32, filepath: Option<PathBuf>) -> Result<LineData> {
    match filepath {
        Some(filepath) => get_lines_from_file(&*filepath),
        None => Ok(get_line_from_cmd(line_number)),
    }
}

fn print_results(l1: &LineData, l2: &LineData, diffs: Vec<Difference>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(prettytable::row![bFgc => l1.name, "Same", l2.name]);
    let iterator = diffs.iter();
    let mut row_index = 0;
    let mut previous: Option<String> = None;
    let column_width = (termwidth() - 8) / 3;

    for d in iterator {
        match d {
            Same(line) => {
                previous = None;
                table.add_row(row!["", fill(line, column_width), ""])
            }
            Add(line) => {
                if let Some(previous_line) = previous {
                    table.remove_row(row_index);
                    row_index -= 1;
                    let new_row = table.add_row(row![
                        fill(&previous_line, column_width),
                        "",
                        fill(line, column_width)
                    ]);
                    previous = None;
                    new_row
                } else {
                    previous = None;
                    table.add_row(row!["", "", fill(line, column_width)])
                }
            }
            Rem(line) => {
                previous = Some(line.to_string());
                table.add_row(row![fill(line, 18), "", ""])
            }
        };
        row_index += 1;
    }
    table.add_row(row![bFgc => l1.length(), "Characters", l2.length()]);
    table.add_row(row![bFgc => l1.number_chunks(), "Chunks", l2.number_chunks()]);
    table.printstd();
}

fn write_output(file: Option<PathBuf>, content: &str) {
    if let Some(file) = &file {
        match File::create(file) {
            Ok(mut file) => {
                if let Err(error) = file.write_all(content.as_bytes()) {
                    println!("couldn't write to {:?}: {:?}", file, error)
                }
            }
            Err(error) => println!("couldn't write to {:?}: {:?}", file, error),
        }
    }
}

/// Comapare two lines with given configuration.
///
/// * `config` - Configuration
pub fn compare_lines(config: Config) -> Result<()> {
    let (mut s1, mut s2) = if let Some(filepath) = config.file {
        verify_existing_file(&*filepath)?;
        get_two_lines_from_file(&*filepath)?
    } else {
        let l1 = if let Some(l1) = config.line1 {
            LineData::new("Line 1", &l1)
        } else {
            get_line(1, config.file1)?
        };
        let l2 = if let Some(l2) = config.line2 {
            LineData::new("Line 2", &l2)
        } else {
            get_line(2, config.file2)?
        };
        (l1, l2)
    };

    //println!("{}: \n{}", s1.name, s1.line);
    //println!("{}: \n{}", s2.name, s2.line);

    s1.preprocess_chunks(&config.separators, config.sort, config.lowercase);
    s2.preprocess_chunks(&config.separators, config.sort, config.lowercase);

    write_output(config.output_file1, &s1.preprocessed);
    write_output(config.output_file2, &s2.preprocessed);

    let changeset = Changeset::new(&s1.preprocessed, &s2.preprocessed, "\n");
    print_results(&s1, &s2, changeset.diffs);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preprocess_no_sorting() {
        let mut data = LineData::new("Line 1", "hello world");
        data.preprocess_chunks(&vec![' '], false, false);
        assert_eq!("hello\nworld", data.preprocessed);

        let mut data = LineData::new("Line 1", "hello world");
        data.preprocess_chunks(&vec![';'], false, false);
        assert_eq!("hello world", data.preprocessed);

        let mut data = LineData::new("Line 1", "hello world");
        data.preprocess_chunks(&vec!['o'], false, false);
        assert_eq!("hell\n w\nrld", data.preprocessed);
    }

    #[test]
    fn preprocess_lowercase() {
        let mut data = LineData::new("Line 1", "hello world");
        data.preprocess_chunks(&vec![' '], false, true);
        assert_eq!("hello\nworld", data.preprocessed);

        let mut data = LineData::new("Line 1", "Hello wOrld");
        data.preprocess_chunks(&vec![';'], false, true);
        assert_eq!("hello world", data.preprocessed);

        let mut data = LineData::new("Line 1", "HELLO WORLD");
        data.preprocess_chunks(&vec!['o'], false, true);
        assert_eq!("hell\n w\nrld", data.preprocessed);
    }

    #[test]
    fn preprocess_sorting() {
        let mut data = LineData::new("Line 1", "a b c");
        data.preprocess_chunks(&vec![' '], true, false);
        assert_eq!("a\nb\nc", data.preprocessed);

        let mut data = LineData::new("Line 1", "c b a");
        data.preprocess_chunks(&vec![' '], true, false);
        assert_eq!("a\nb\nc", data.preprocessed);
    }

    #[test]
    fn preprocess_multiple_separators() {
        let mut data = LineData::new("Line 1", "a b;c");
        data.preprocess_chunks(&vec![' '], true, false);
        assert_eq!("a\nb;c", data.preprocessed);

        let mut data = LineData::new("Line 1", "c b a");
        data.preprocess_chunks(&vec![' ', ';'], true, false);
        assert_eq!("a\nb\nc", data.preprocessed);
    }

    #[test]
    fn read_one_line() -> Result<()> {
        let l1 = get_lines_from_file(Path::new("examples/test.txt"))?;
        assert_eq!("test.txt", l1.name);
        assert_eq!("Hello world 1 3 .\nas the %+3^ night", l1.line);
        Ok(())
    }

    #[test]
    fn read_two_lines() -> Result<()> {
        let (l1, l2) = get_two_lines_from_file(Path::new("examples/test.txt"))?;
        assert_eq!("Line 1", l1.name);
        assert_eq!("Line 2", l2.name);
        assert_eq!("Hello world 1 3 .", l1.line);
        assert_eq!("as the %+3^ night", l2.line);
        Ok(())
    }
}
