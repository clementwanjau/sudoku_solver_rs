# Sudoku Solver
A basic sudoku solver using the wave function collapse algorithm.
This program can solve a sudoku of any dimensions other than the standard 9x9 grid.

The input is read from a txt file that is passed in as an argument to the program. The text in the input file is formatted as follows.
- Rows in the grid are separated by new lines, 
- Columns are separated by commas,
- The blank spots in the grid are set to zero.

An example formatted grid is:

```
    3, 2, 0, 4
    0, 3, 0, 0
    0, 1, 0, 0
    0, 0, 3, 2
```

### Usage
``sudoku_solver_rs --file <file_path>``

The solved puzzle is output to a file with the name `<file_name>_solution.txt` in the same folder.

### Author
Clement Wanjau <clementwanjau@yahoo.com>