CLI for resolving sudoku puzzles.
Excercise for studying rust.
Look at [samples](/samples/) to review i/o format for puzzles. Can do both stdi/o and files.


```
Sudoku Puzzle solver.
Input is a board formatted in the following manner:
Each cell is designated by a digit:
    - 1-9 for assigned value
    - 0 for empty value
separators between digits are not required, though
you can separate lines with newlines.

Usage: rust-sudoku-quick-solver.exe [OPTIONS]

Options:
  -i, --input-file <INPUT_FILE>    input file path (defaults to stdin)
  -o, --output-file <OUTPUT_FILE>  output file path (defaults to stdout)
  -h, --help                       Print help
  -V, --version                    Print version
```
