// https://www.reddit.com/r/dailyprogrammer/comments/oirb5v/20210712_challenge_398_difficult_matrix_sum/
// Calculate the largest possible sum made from an nxn matrix where only 1 number can be taken from each row and column

// Hungarian algorithm
// https://www.youtube.com/watch?v=FCaD34z--bY

fn main() {
    let mut org_matrix: [u32; 25] = [
        123456789,   752880530,   826085747,  576968456,   721429729,
        173957326,   1031077599,  407299684,  67656429,    96549194,
        1048156299,  663035648,   604085049,  1017819398,  325233271,
        942914780,   664359365,   770319362,  52838563,    720059384,
        472459921,   662187582,   163882767,  987977812,   394465693,
    ];

    let mut matrix = org_matrix;

    // Hungarian algorithm is for minimum matching so invert all values
    // change to false for minimum sum
    let maximum = true;
    if maximum {
        let max = *matrix.iter().max().unwrap();
        for i in 0..matrix.len() {
            matrix[i] = max - matrix[i];
        }
    }

    matrix = sub_row_col(matrix);

    let mut data = min_lines(matrix);
    // Not enough lines to cover the whole matrix
    // while (data[1].len() + data[2].len()) < 5 {
    for i in 0..3 {
        matrix = adjust_matrix(matrix, &data[1], &data[2]);
        data = min_lines(matrix);
    }

    let mut maximum: u32 = 0;
    for i in 0..25 {
        if data[0].contains(&i) {
            maximum += org_matrix[i as usize];
        }
    }

    print_matrix(&org_matrix, &data[0]);
    println!("Maximum sum is {}", maximum);
}

fn print_matrix(matrix: &[u32; 25], marks: &Vec<u32>) {
    for i in 0..5 {
        for j in 0..5 {
            if marks.contains(&((i * 5 + j) as u32)) {
                let s = matrix[i * 5 + j].to_string();
                let gap = " ".repeat(10 - s.len());
                print!("[{}]{}", s, gap);
            } else {
                let s = matrix[i * 5 + j].to_string();
                let gap = " ".repeat(12 - s.len());
                print!("{}{}", s, gap);
            }
        }
        println!();
    }
}

// every row and column has its internal minimum value subtracted
fn sub_row_col(mut matrix: [u32; 25])-> [u32; 25] {
    // row
    let max = *matrix.iter().max().unwrap();
    for i in 0..5 {
        let start_row = i * 5;
        let end_row = start_row + 5;
        let mut min = max;
        for j in start_row..end_row {
            if matrix[j] < min {
                min = matrix[j];
            }
        }

        for j in start_row..end_row {
            matrix[j] -= min;
        }
    }

    // column
    for i in 1..6 {
        let col_mod = i % 5;
        let mut min = max;
        for j in 0..5 {
            if matrix[j * 5 + col_mod] < min {
                min = matrix[j * 5 + col_mod];
            }
        }

        if min == 0 {
            continue;
        }

        for j in 0..5 {
            matrix[j * 5 + col_mod] -= min;
        }
    }

    return matrix;
}

// Find minimum number of lines to cover all 0 elements
// First returned vector contains the marked indexes
// Second returned vector contains the covered rows
// Third returned vector contains the covered columns
fn min_lines(matrix: [u32; 25]) -> Vec<Vec<u32>>{
    let mut bool_matrix = matrix.map(|x| if x == 0 {true} else {false});
    let mut idxs: Vec<u32> = Vec::new();

    for j in 0..5 {
        // find row with minimum number of true elements
        let mut min_true: [u32; 2] = [0, 6];
        for i in 0..5 {
            let start_row = i * 5;
            let end_row = start_row + 5;
            let mut trues = 0;
            for j in start_row..end_row {
                if bool_matrix[j] {
                    trues += 1;
                }
            }

            if trues < min_true[1] && trues > 0 {
                min_true = [i as u32, trues];
            }
        }

        // find index of first true element in row
        for i in (min_true[0] * 5)..(min_true[0] * 5 + 5) {
            if bool_matrix[i as usize]{
                idxs.push(i);
                break;
            }
        }

        // set row and column of that index to false
        // row
        for i in min_true[0] * 5..min_true[0] * 5 + 5 {
            bool_matrix[i as usize] = false;
        }

        // column
        let col_mod = if idxs.len() <= j {0} else {idxs[j] % 5};
        for k in 0..5 {
            bool_matrix[k * 5 + col_mod as usize] = false;
        }
    }

    // Rows with no marked 0s
    let mut non_marked_row: [bool; 5] = [false; 5];
    for i in 0..5 {
        let mut no_zeros = true;
        for j in 0..5 {
            if idxs.contains(&(i * 5 + j)) {
                no_zeros = false;
                break;
            }
        }

        if no_zeros {
            non_marked_row[i as usize] = true;
        }
    }

    // Reset bool matrix
    bool_matrix = matrix.map(|x| if x == 0 {true} else {false});

    // Marked columns
    // If a matching index exists in an unmarked row, the corresponding column index is saved to marked_cols
    let mut marked_cols: Vec<u32> = Vec::new();
    for i in 0..5 {
        if non_marked_row[i] {
            for j in 0..5 {
                let idx = (i * 5 + j) as u32;
                if bool_matrix[idx as usize] && !idxs.contains(&idx) {
                    marked_cols.push(j as u32);
                }
            }
        }
    }

    // Check marked_cols and marked_rows for repeats
    for y in marked_cols.iter() {
        for i in idxs.iter() {
            if *y == i % 5 {
                non_marked_row[(i / 5) as usize] = true;
            }
        }
    }

    // Generate marked rows
    let mut marked_rows: Vec<u32> = Vec::new();
    for i in 0..non_marked_row.len() {
        if !non_marked_row[i] {
            marked_rows.push(i as u32);
        }
    }

    return vec!(idxs, marked_rows, marked_cols);
}

fn adjust_matrix(mut matrix: [u32; 25], rows: &Vec<u32>, columns: &Vec<u32>) -> [u32; 25] {
    // Get global non-covered minimum
    let mut min: u32 = *matrix.iter().max().unwrap();
    for i in 0..25 {
        if !columns.contains(&(i % 5)) && !rows.contains(&(i / 5)) {
            if matrix[i as usize] < min {
                min = matrix[i as usize];
            }
        }
    }

    // Subtract min from all uncovered items
    for i in 0..25 {
        if !columns.contains(&(i % 5)) && !rows.contains(&(i / 5)) {
            matrix[i as usize] -= min;
        }
    }

    // Add min to all items that are covered twice
    for i in 0..25 {
         if columns.contains(&(i % 5)) && rows.contains(&(i / 5)) {
             matrix[i as usize] += min;
         }
    }
    print_matrix(&matrix, &vec![]);
    println!("");

    return matrix;
}
