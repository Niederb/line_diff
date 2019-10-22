extern crate difference;
#[macro_use]
extern crate clap;

use clap::{App, Arg};
use std::fs::File;
use std::io::{BufRead, BufReader};

use difference::Changeset;
use difference::Difference;
use difference::Difference::{Same, Add, Rem};

#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

fn get_line_from_file(filename: &str) -> String {
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

fn get_lines_from_file(filename: &str) -> (String, String) {
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

fn get_lines_from_cmd() -> (String, String) {
    //let mut buffer = String::new();
    //io::stdin().read_to_string(&mut buffer)?;
    ("".to_owned(), "".to_owned())
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

fn main() {
    let matches = App::new("Line diff")
        .version(crate_version!())
        .author("Thomas Niederberger <thomas@niederb.ch>")
        .about("Compare two lines")
        .arg(
            Arg::with_name("file")
                .short("f")
                .help("File containing the two lines to compare")
                .takes_value(true),
        )
		.arg(
            Arg::with_name("file1")
                .short("x")
                .help("File containing the first line for comparison")
                .takes_value(true),
        )
		.arg(
            Arg::with_name("file2")
                .short("y")
                .help("File containing the second line for comparison")
                .takes_value(true),
        )
		.arg(
            Arg::with_name("separator")
                .short("s")
                .help("Separator for splitting lines")
                .takes_value(true),
        )
        .get_matches();

    let (s1, s2) = if matches.is_present("file") {
        let input_file = matches.value_of("file").unwrap_or("");
        get_lines_from_file(input_file)
    } else if matches.is_present("file1") && matches.is_present("file2") {
		let input_file = matches.value_of("file1").unwrap_or("");
        let s1 = get_line_from_file(input_file);
		let input_file = matches.value_of("file2").unwrap_or("");
        let s2 = get_line_from_file(input_file);
		(s1, s2)
	} else {
        get_lines_from_cmd()
    };

    let separator = matches.value_of("separator").unwrap_or(";");
    let s1 = s1.replace(separator, "\n");
    let s2 = s2.replace(separator, "\n");

    let separator = ",";
    let s1 = s1.replace(separator, "\n");
    let s2 = s2.replace(separator, "\n");

    let changeset = Changeset::new(&s1, &s2, "\n");
    println!("Newline");
    println!("{}", changeset);

    for c in &changeset.diffs {
        println!("{:?}", c);
    }

    print_results(changeset.diffs);
}
