fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            new_matrix[i][j] = matrix[j][i];
        }
    }
    new_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    let mut formatted_matrix = String::new();
    for (i, row) in matrix.iter().enumerate() {
        let pair = match i {
            0 => ('⎡', '⎤'),
            1 => ('|', '|'),
            2 => ('⎣', '⎦'),
            _ => panic!("Invalid"),
        };
        formatted_matrix.push(pair.0);
        print!("{}", pair.0);
        for (j, num) in row.iter().enumerate() {
            if j == matrix[0].len() - 1 {
                print!("{}", num);
            } else {
                print!("{} ", num);
            }
        }
        print!("{}", pair.1);
        println!()
    }
    // println!("{:?}", formatted_matrix)
}
fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
