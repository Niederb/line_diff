extern crate difference;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::path::Path;

use difference::Changeset;
use difference::Difference;
use difference::Difference::{Add, Rem, Same};

#[macro_use]
extern crate prettytable;
use prettytable::Table;

fn get_line_from_file(filename: &Path) -> String {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut s = "".to_owned();
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if index == 0 {
            s = line.to_owned();
        } else {
            println!("File contains additional lines that will be ignored");
            break;
        }
    }
    s.to_string()
}

fn get_lines_from_file(filename: &Path) -> (String, String) {
    let file = File::open(filename).unwrap();
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
    (s1.to_string(), s2.to_string())
}

fn get_line_from_cmd(line_number: i32) -> String {
    println!("Please provide line #{}: ", line_number);
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("");
    buffer.trim().to_string()
}

fn get_line(line_number: i32, filepath: Option<&Path>) -> String {
    match filepath {
        Some(filepath) => get_line_from_file(filepath),
        None => get_line_from_cmd(line_number),
    }
}

fn print_results(diffs: Vec<Difference>) {
    let mut table = Table::new();
    table.add_row(row!["L1", "Same", "L2"]);
    for d in diffs {
        match d {
            Same(line) => table.add_row(row!["", line, ""]),
            Add(line) => table.add_row(row!["", "", line]),
            Rem(line) => table.add_row(row![line, "", ""]),
        };
    }
    table.printstd();
}

fn preprocess_chunks(s: &str, separator: &[char], sort: bool) -> String {
    let mut chunks: Vec<&str> = s.split(|c| separator.contains(&c)).collect();
    if sort {
        chunks.sort();
    }
    chunks.join("\n")
}

fn main() {
    let matches = App::new("Line diff")
        .version(crate_version!())
        .author(crate_authors!())
        .about("Compare two lines by splitting the lines into smaller chunks.")
        .arg(
            Arg::with_name("file1")
                .help("File containing the first line for comparison")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("file2")
                .help("File containing the second line for comparison")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("separator")
                .short("s")
                .help("Separator for splitting lines")
                .takes_value(true)
                .multiple(true),
        )
        .arg(
            Arg::with_name("sorted")
                .short("o")
                .help("Whether or not the values should be sorted before comparing."),
        )
        .get_matches();

    let (s1, s2) = if let Some(filepath1) = matches.value_of("file1") {
        let path_file1 = Path::new(filepath1);
        if !path_file1.exists() || !path_file1.is_file() {
            println!("Cannot find file1: {}", filepath1);
            return;
        }
        if let Some(filepath2) = matches.value_of("file2") {
            let path_file2 = Path::new(filepath2);
            if !path_file2.exists() || !path_file2.is_file() {
                println!("Cannot find file1: {}", filepath2);
                return;
            }
            let s1 = get_line(1, Some(path_file1));
            let s2 = get_line(1, Some(path_file2));
            (s1, s2)
        } else {
            get_lines_from_file(path_file1)
        }
    } else {
        let s1 = get_line(1, None);
        let s2 = get_line(2, None);
        (s1, s2)
    };

    let sort = matches.is_present("sorted");

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
    println!("Line 1: \n{}", s1);
    println!("Line 2: \n{}", s2);

    let s1 = preprocess_chunks(&s1, &separator_chars, sort);
    let s2 = preprocess_chunks(&s2, &separator_chars, sort);

    let changeset = Changeset::new(&s1, &s2, "\n");
    print_results(changeset.diffs);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preprocess_no_sorting() {
        let output = preprocess_chunks("hello world", &vec![' '], false);
        assert_eq!("hello\nworld", output);
        let output = preprocess_chunks("hello world", &vec![';'], false);
        assert_eq!("hello world", output);
        let output = preprocess_chunks("hello world", &vec!['o'], false);
        assert_eq!("hell\n w\nrld", output);
    }

    #[test]
    fn preprocess_sorting() {
        let output = preprocess_chunks("a b c", &vec![' '], true);
        assert_eq!("a\nb\nc", output);
        let output = preprocess_chunks("c b a", &vec![' '], true);
        assert_eq!("a\nb\nc", output);
    }

    #[test]
    fn preprocess_multiple_separators() {
        let output = preprocess_chunks("a b;c", &vec![' '], true);
        assert_eq!("a\nb;c", output);
        let output = preprocess_chunks("c b a", &vec![' ', ';'], true);
        assert_eq!("a\nb\nc", output);
    }

    #[test]
    fn read_one_line() {
        let l1 = get_line_from_file(Path::new("test.txt"));
        assert_eq!("Hello world 1 3 .", l1);
    }

    #[test]
    fn read_two_lines() {
        let (l1, l2) = get_lines_from_file(Path::new("test.txt"));
        assert_eq!("Hello world 1 3 .", l1);
        assert_eq!("as the %+3^ night", l2);
    }
}
