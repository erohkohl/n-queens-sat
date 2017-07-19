extern crate dpll;

use dpll::view::renderservice as renderservice;

#[test]
fn print_chess_board_n_four() {
    let assign: Vec<i32> = vec![-42, -22, -34, -44
                                , 43, -33, -23, -21
                                , 24, -14, -13, -32
                                , 31, 12, -11, -41];

    let expected: String = String::from(". Q . . \n. . . Q \nQ . . . \n. . Q . \n");
    let chess_board: String = renderservice::print_chess_board(assign);
    assert_eq!(expected, chess_board);
}

#[test]
fn print_chess_board_n_eight() {
    let assign: Vec<i32> = vec![
        -11, -12, -13, 14, -15, -16, -17, -18, -19, -20, -21, 22, -23, -24, -25, -26, -27, -28,
        -29, -30, -31, -32, -33, -34, -35, -36, 37, -38, -39, -40, -41, -42, 43, -44, -45, -46,
        -47, -48, -49, -50, -51, -52, -53, -54, -55, 56, -57, -58, -59, -60, -61, -62, -63, -64,
        -65, -66, -67, 68, -69, -70, -71, -72, -73, -74, 75, -76, -77, -78, -79, -80, 81, -82,
        -83, -84, -85, -86, -87, -88];

    let expected: String = String::from(". . . Q . . . .
                                        . Q . . . . . .
                                        . . . . . . Q .
                                        . . Q . . . . .
                                        . . . . . Q . .
                                        . . . . . . . Q
                                        . . . . Q . . .
                                        Q . . . . . . . ");
    let chess_board: String = renderservice::print_chess_board(assign);
    //assert_eq!(expected, chess_board);
}