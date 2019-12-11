# line_diff
Tool to make a diff between to single lines. The intended use case is to compare long lines where parts are different or missing.
For example:
* long command lines with many arguments and flags
* Compiler commands with many paths (with potentially different order)
* Long function declarations with slightly different arguments

## Example output
```
Line 1: 
cargo run -- -o --file l1.txt -s " ;"
Line 2:
cargo run --release -- --file l1.txt -s " ;" -o
┌────────┬────────────┬───────────┐
│ Line 1 │    Same    │  Line 2   │
├────────┼────────────┼───────────┤
│        │ "          │           │
│        │ --         │           │
│        │ --file     │           │
├────────┼────────────┼───────────┤
│        │            │ --release │
├────────┼────────────┼───────────┤
│        │ -o         │           │
│        │ -s         │           │
│        │ ;"         │           │
│        │ cargo      │           │
│        │ l1.txt     │           │
│        │ run        │           │
├────────┼────────────┼───────────┤
│   37   │ Characters │    47     │
├────────┼────────────┼───────────┤
│   9    │   Chunks   │    10     │
└────────┴────────────┴───────────┘
```

## Examples
Compare two lines from two different input files.
```
line_diff l1.txt l2.txt
```

Compare two lines from two different input files. With the -o option the chunks will be sorted before comparison.
This is handy for cases such as compiler flags where the ordering does not matter.
```
line_diff l1.txt l2.txt -o
```

Compare two lines from two a single input file and with sorting of the chunks. 
Specify two different separators (' ' and ';') with the -s option
```
line_diff --file l1.txt -o -s "; "
```

Compare two lines by specifying the string on the command line
```
line_diff --line1 "hello world" --line2 "hello there"
```