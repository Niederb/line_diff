extern crate difference;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::error;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use difference::Changeset;
use difference::Difference;
use difference::Difference::{Add, Rem, Same};

#[macro_use]
extern crate prettytable;
use prettytable::format;
use prettytable::Table;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

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

fn get_line_from_file(path: &Path) -> Result<LineData> {
    verify_existing_file(path)?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut s = "".to_owned();
    for (index, line) in reader.lines().enumerate() {
        let line = line?;

        if index == 0 {
            s = line.to_owned();
        } else {
            println!("File contains additional lines that will be ignored");
            break;
        }
    }
    let file_name = if let Some(file_name2) = path.file_name() {
        if let Ok(file_name3) = file_name2.to_os_string().into_string() {
            file_name3
        } else {
            "".into()
        }
    } else {
        "".into()
    };
    Ok(LineData::new(&file_name, &s))
}

fn get_lines_from_file(path: &Path) -> Result<(LineData, LineData)> {
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
    io::stdin().read_line(&mut buffer).expect("");
    LineData::new(&format!("Line {}", line_number), &buffer.trim().to_string())
}

fn get_line(line_number: i32, filepath: Option<&str>) -> Result<LineData> {
    match filepath {
        Some(filepath) => get_line_from_file(Path::new(filepath)),
        None => Ok(get_line_from_cmd(line_number)),
    }
}

fn print_results(l1: &LineData, l2: &LineData, diffs: Vec<Difference>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_BOX_CHARS);
    table.add_row(row![bFgc => l1.name, "Same", l2.name]);
    let iterator = diffs.iter();
    let mut row_index = 0;
    let mut previous: Option<String> = None;
    for d in iterator {
        match d {
            Same(line) => {
                previous = None;
                table.add_row(row!["", line, ""])
            }
            Add(line) => {
                if let Some(previous_line) = previous {
                    table.remove_row(row_index);
                    row_index -= 1;
                    let new_row = table.add_row(row![previous_line, "", line]);
                    previous = None;
                    new_row
                } else {
                    previous = None;
                    table.add_row(row!["", "", line])
                }
            }
            Rem(line) => {
                previous = Some(line.to_string());
                table.add_row(row![line, "", ""])
            }
        };
        row_index += 1;
    }
    table.add_row(row![bFgc => l1.length(), "Characters", l2.length()]);
    table.add_row(row![bFgc => l1.number_chunks(), "Chunks", l2.number_chunks()]);
    table.printstd();
}

fn main() -> Result<()> {
    let matches = App::new("Line diff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Compare two lines by splitting the lines into smaller chunks and comparing the chunks. \
        There are multiple ways of specifying the two lines: \n \
        \ta single file that contains the two lines (--file option) \n \
        \tspecifying the two lines separately as a file path (indexed argument 1 and 2), on the command line (--line1 and --line2) or using command line input.")
        .arg(
            Arg::with_name("file")
                .long("file")    
                .help("A single file containing two lines. Remaining lines will be ignored.")
                .short("f")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file1")
                .help("Path to file containing the first line. Remaining lines will be ignored.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file2")
                .help("Path to file containing the second line. Remaining lines will be ignored.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("line1")
                .short("x")
                .long("line1")
                .help("First line as string")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("line2")
                .short("y")
                .long("line2")
                .help("Second line as string")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .help("Separator for splitting lines. It is possible to define multiple separators.")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("sorted")
                .short("o")
                .help("Whether or not the chunks should be sorted before comparing."),
        )
        .arg(
            Arg::with_name("lowercase")
                .short("l")
                .help("Convert the chunks to lowercase before converting"),
        )
        .get_matches();

    let (mut s1, mut s2) = if let Some(filepath) = matches.value_of("file") {
        let path_file = Path::new(filepath);
        verify_existing_file(path_file)?;
        get_lines_from_file(path_file)?
    } else {
        let l1 = if let Some(l1) = matches.value_of("line1") {
            LineData::new("Line 1", &l1)
        } else {
            get_line(1, matches.value_of("file1"))?
        };
        let l2 = if let Some(l2) = matches.value_of("line2") {
            LineData::new("Line 2", &l2)
        } else {
            get_line(2, matches.value_of("file2"))?
        };
        (l1, l2)
    };

    let sort = matches.is_present("sorted");
    let lowercase = matches.is_present("lowercase");

    let separator_chars = if matches.is_present("separator") {
        let separators = matches.values_of("separator").unwrap().collect::<Vec<_>>();
        let mut separator_chars: Vec<char> = Vec::new();
        for s in separators {
            println!("Separator: '{}'", s);
            for character in s.chars() {
                separator_chars.push(character);
            }
        }
        separator_chars
    } else {
        vec![' ']
    };
    println!("{}: \n{}", s1.name, s1.line);
    println!("{}: \n{}", s2.name, s2.line);

    s1.preprocess_chunks(&separator_chars, sort, lowercase);
    s2.preprocess_chunks(&separator_chars, sort, lowercase);

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
        let l1 = get_line_from_file(Path::new("test.txt"))?;
        assert_eq!("test.txt", l1.name);
        assert_eq!("Hello world 1 3 .", l1.line);
        Ok(())
    }

    #[test]
    fn read_two_lines() -> Result<()> {
        let (l1, l2) = get_lines_from_file(Path::new("test.txt"))?;
        assert_eq!("Line 1", l1.name);
        assert_eq!("Line 2", l2.name);
        assert_eq!("Hello world 1 3 .", l1.line);
        assert_eq!("as the %+3^ night", l2.line);
        Ok(())
    }
}
