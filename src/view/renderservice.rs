pub fn print_chess_board(mut assign:Vec<i32>){
    assign.sort();
    for i in assign{
        print!("{}", i);
    }
}