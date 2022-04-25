mod gameboard;


fn main() {
    println!("Hello, world!");
    let b = gameboard::Board::new(10,10,10);
    b.print();
    b.print();
}
