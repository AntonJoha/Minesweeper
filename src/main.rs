mod gameboard;


fn main() {
    let mut b = gameboard::Board::new(9,9,10);
    let over = b.guess(5,5);
    b.print();
    println!("{}", over);
}
