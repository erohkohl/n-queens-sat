extern crate dpll;

use dpll::view::renderservice as renderservice;

#[test]
fn print_chess_board_test(){
    let assign:Vec<i32> = vec![-42, -22, -34, -44
                               , 43, -33, -23, -21
                               , 24, -14, -13, -32
                               , 31, 12, -11, -41];

    let expected:String = String::from(". Q . . \n. . . Q \nQ . . . \n. . Q . \n");
    let chess_board:String = renderservice::print_chess_board(assign);
    assert_eq!(expected, chess_board);
}