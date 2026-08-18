```markdown
# N-Queens Solver in Rust

A lightweight, backtracking implementation of the classic **N-Queens Problem** written in Rust.

## Overview

The **N-Queens Problem** is a classic combinatorial puzzle where the goal is to place $N$ chess queens on an $N \times N$ chessboard such that no two queens attack each other. This means no two queens can share the same row, column, or diagonal.

This project implements a backtracking algorithm that explores valid board configurations row-by-row and calculates the total number of distinct solutions for a given $N$.

---

## How It Works

The program builds the solution incrementally using depth-first search (DFS) with backtracking:

1. **Row-by-Row Placement (`place_queen`)**: 
   The algorithm attempts to place one queen per row, starting from `row 0` up to `row N - 1`.
2. **Safety Verification (`check_position`)**: 
   Before placing a queen at position `(row, col)`, the solver checks if any previously placed queen can attack this position:
   - **Vertical check**: Checks if another queen exists in the same column above the current row (`xr < row`).
   - **Upper-Left Diagonal check**: Traverses diagonally up and left (`row - 1, col - 1`).
   - **Upper-Right Diagonal check**: Traverses diagonally up and right (`row - 1, col + 1`).
3. **Backtracking**: 
   If placing a queen leads to a state where no valid square exists in subsequent rows, the algorithm unsets the current queen (`matrix[row][col] = 0`) and tries the next column in the current row.
4. **Solution Count**: 
   When `row == n`, a valid configuration has been reached. The global counter is incremented, and the search backtracks to find remaining solutions.

---

## Code Breakdown

### Key Functions

* `check_position(matrix, row, col, n) -> bool`
  - Validates whether placing a queen at `(row, col)` is safe.
  - Only checks rows above the current row (since lower rows haven't been populated yet).

* `place_queen(matrix, row, n, count)`
  - Recursive helper function that drives the backtracking algorithm.
  - Base Case: `row == n` increments `*count`.
  - Recursive Case: Iterates through columns `0..n`, tests safety, mutates the matrix, recurses to `row + 1`, and backtracks upon return.

* `main()`
  - Reads input $N$ from `stdin`.
  - Initializes an $N \times N$ matrix populated with zeroes (`0` = empty, `1` = queen).
  - Triggers `place_queen` and outputs the total number of valid solutions.

---

## Usage

### Prerequisites
* [Rust toolchain](https://www.rust-lang.org/tools/install) (`cargo` and `rustc`).

### Building and Running

1. Clone or save the repository:
   ```bash
   git clone [https://github.com/your-username/n-queens-rust.git](https://github.com/your-username/n-queens-rust.git)
   cd n-queens-rust

```

2. Compile and run using Cargo:
```bash
cargo run

```


*Alternatively, compile directly with `rustc`:*
```bash
rustc main.rs
./main

```


3. Enter the number of queens when prompted:
```text
Enter the number of queens: 8
The number of Sol is possible: 92

```



---

## Example Outputs

| Board Size ($N$) | Number of Valid Solutions |
| --- | --- |
| $1$ | 1 |
| $2$ | 0 |
| $3$ | 0 |
| $4$ | 2 |
| $8$ | 92 |
| $10$ | 724 |

---

## Complexity Analysis

* **Time Complexity**: $\mathcal{O}(N!)$ — The search space is bounded by factorial growth as each added row reduces available valid candidate columns.
* **Space Complexity**: $\mathcal{O}(N^2)$ — Used for storing the $N \times N$ matrix on the heap, plus $\mathcal{O}(N)$ call stack depth for recursion.

```

```