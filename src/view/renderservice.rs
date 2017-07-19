/// This method gets a satisfiable assignment of the n-queens problem and returns an String,
/// which contains the queens positions on a chess board.
pub fn print_chess_board(assign: Vec<i32>) -> String {
    let mut chess_board: String = String::new();
    let mut queens_position: Vec<i32> = Vec::new();
    let index: i32 = (assign.len() as f64).sqrt() as i32;

    for i in assign {
        if i > 0 {
            queens_position.push(i);
        }
    }
    let mut col: Vec<i32> = Vec::new();
    let mut row: Vec<i32> = Vec::new();

    for i in queens_position {
        col.push(i % 10i32);
        row.push(i / 10i32);
    }

    for i in 1..index + 1 {
        let index_col: i32 = row.iter().position(|&r| r == i).unwrap() as i32;

        for j in 1..index + 1 {
            if j == *col.get(index_col as usize).unwrap() {
                chess_board.push_str("Q ");
            } else {
                chess_board.push_str(". ");
            }
        }
        chess_board.push_str("\n");
    }
    return chess_board;
}