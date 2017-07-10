extern crate dpll;

use dpll::view::renderservice as renderservice;

#[test]
fn print_chess_board_test(){
    let assign:Vec<i32> = vec![
        -33, -43, -31, -42, -23,
        13, -14, -12, -24, -11,
        -11, -24, -11, -12, -24,
        -11, 22, -21, 41, -44, 34,
        -32];
    renderservice::print_chess_board(assign);

}