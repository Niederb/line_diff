# line_diff

![Rust](https://github.com/Niederb/line_diff/workflows/build/badge.svg)
Tool to compare two single lines of text. The intended use case is to compare long lines where parts are different or missing.
Example use cases:

- long command lines with many arguments and flags
- Compiler commands with many paths (with potentially different order)
- Long function declarations with slightly different arguments

## Features

- Multiple, user specified separators
- Converting all text to lowercase
- Sorting chunks before comparing the chunks
- Different input options: Command line, two files, single file or standard input
- Statistics about the number of chunks and number of characters
- Store preprocessed data into files in order to use external diff tool

## Example output

Comparing to different cargo commands with arguments in different order.
With line_diff it is easy to spot that the only difference is the --release argument
```
Line 1: 
cargo run -- -o --file f1.txt -s ",;"
Line 2:
cargo run --release -- --file f1.txt -s ",;" -o
┌────────┬────────────┬───────────┐
│ Line 1 │    Same    │  Line 2   │
├────────┼────────────┼───────────┤
│        │ ",;"       │           │
│        │ --         │           │
│        │ --file     │           │
├────────┼────────────┼───────────┤
│        │            │ --release │
├────────┼────────────┼───────────┤
│        │ -o         │           │
│        │ -s         │           │
│        │ cargo      │           │
│        │ f1.txt     │           │
│        │ run        │           │
├────────┼────────────┼───────────┤
│   37   │ Characters │    47     │
├────────┼────────────┼───────────┤
│   8    │   Chunks   │     9     │
└────────┴────────────┴───────────┘
```

## Examples

Compare two lines from two different input files.
```
line_diff f1.txt f2.txt
```

Compare two lines from two different input files. With the -o option the chunks will be sorted before comparison.
This is handy for cases such as compiler flags where the ordering does not matter.
```
line_diff f1.txt f2.txt -o
```

Compare two lines from two a single input file and with sorting of the chunks. 
Specify two different separators (' ' and ';') with the -s option
```
line_diff --file f1.txt -o -s ' ' ';'
```

Compare two lines by specifying the string on the command line
```
line_diff --line1 "hello world" --line2 "hello there"
```

Compare two lines, but first convert them both to lowercase
```
line_diff --line1 "hello world" --line2 "Hello wOrld" -l
```
