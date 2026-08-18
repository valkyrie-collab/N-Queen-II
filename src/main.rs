use std::io::{ self, Write };

fn check_position(matrix: &Vec<Vec<u32>>, row: i32, col: i32, n: i32) -> bool {
    
    for xr in 0..row as usize {

        if matrix[xr][col as usize] == 1 {
            return false;
        }

    }

    let mut xr: i32 = row - 1;
    let mut xc: i32 = col - 1;

    while xr >= 0 && xc >= 0 {

        if matrix[xr as usize][xc as usize] == 1 {
            return false;
        }

        xr -= 1;
        xc -= 1;
    }

    xr = row - 1;
    xc = col + 1;

    while xr >= 0 && xc < n {

        if matrix[xr as usize][xc as usize] == 1 {
            return false;
        }

        xr -= 1;
        xc += 1;
    }

    true
}

fn place_queen(matrix: &mut Vec<Vec<u32>>, row: i32, n: i32, count: &mut i32) {

    if row == n {
        *count += 1;
        return;
    }

    for xc in 0..n as usize {

        if check_position(matrix, row, xc as i32, n) {
            matrix[row as usize][xc] = 1;
            place_queen(matrix, row + 1, n, count);
            matrix[row as usize][xc] = 0;
        }

    }

}

fn main() {
    let mut buf: String = String::new();

    print!("Enter the number of queens: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buf).unwrap();

    let n: i32 = if let Ok(xn) = buf.trim().parse() { xn } else { return; };
    let mut matrix: Vec<Vec<u32>> = vec![vec![0; n as usize]; n as usize];
    let mut count: i32 = 0;

    place_queen(&mut matrix, 0, n, &mut count);

    println!("The number of Sol is possible: {}", count);
}
