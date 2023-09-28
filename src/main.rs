fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    // Create an empty matrix with given size filled with zeros
    let mut matrix = vec![vec![0; size as usize]; size as usize];

    // Initialize boundaries of the matrix to traverse
    let mut top = 0;
    let mut bottom = size as usize;
    let mut left = 0;
    let mut right = size as usize;

    let mut num = 1; // The starting number

    while top < bottom && left < right {
        // Move from left to right
        for i in left..right {
            matrix[top][i] = num;
            num += 1;
        }
        top += 1;

        // Move from top to bottom
        for i in top..bottom {
            matrix[i][right - 1] = num;
            num += 1;
        }
        right -= 1;

        // Move from right to left
        if top < bottom {
            for i in (left..right).rev() {
                matrix[bottom - 1][i] = num;
                num += 1;
            }
            bottom -= 1;
        }

        // Move from bottom to top
        if left < right {
            for i in (top..bottom).rev() {
                matrix[i][left] = num;
                num += 1;
            }
            left += 1;
        }
    }

    matrix
}

fn main() {
    let size: u32 = 33;
    let matrix: Vec<Vec<u32>> = spiral_matrix(size);
    for row in &matrix {
        for &item in row {
            print!("{:4} ", item);
        }
        println!();
    }
}

