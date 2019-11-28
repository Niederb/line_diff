# line_diff
Tool to make a diff between to single lines

## Examples
Compare two lines from two different input files.
line_diff l1.txt l2.txt

Compare two lines from two different input files. With the -o option the chunks will be sorted before comparison.
This is handy for cases such as compiler flags where the ordering does not matter.
line_diff l1.txt l2.txt -o

Compare two lines from two a single input file. With the -o option the chunks will be sorted before comparison.
line_diff l1.txt -o