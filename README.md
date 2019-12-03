# line_diff
Tool to make a diff between to single lines

## Example output
```
Line 1:
a b c d e f
Line 2:
b d f e c2
+----+------+----+
| L1 | Same | L2 |
+----+------+----+
| a  |      |    |
+----+------+----+
|    | b    |    |
+----+------+----+
| c  |      |    |
+----+------+----+
|    |      | c2 |
+----+------+----+
|    | d    |    |
|    | e    |    |
|    | f    |    |
+----+------+----+
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