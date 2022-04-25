use rand::Rng;


pub struct Board {
    pubBoard: Vec<Vec<i32>>,
    xSize: i32,
    YSize: i32,
    board: Vec<Vec<i32>>,
    bombs: i32,
}

fn inc_board(mut board: Vec<Vec<i32>>, x: i32, y: i32, xSize: i32, ySize: i32) -> Vec<Vec<i32>> {
    let offset = vec![-1, 0, 1];

    for i in 0..3 {
        for j in 0..3 {
            if x + offset[i] < xSize && y + offset[j] < ySize {
                if (x + offset[i] >= 0 && y + offset[j] >= 0) {
                    if (board[(x + offset[i]) as usize][(y + offset[j]) as usize] != -1) {
                        board[(x + offset[i]) as usize][(y + offset[j]) as usize] += 1;
                    }
                }
            }
        }
    }
    board
}

fn add_numbers(mut board: Vec<Vec<i32>>, xSize: i32, ySize: i32) -> Vec<Vec<i32>> {
    for x in 0..xSize {
        for y in 0..ySize {
            if board[x as usize][y as usize] == -1 {
                board = inc_board(board, x, y, xSize, ySize);
            }
        }
    }
    board
}

fn empty_board(xSize: i32, ySize: i32) -> Vec<Vec<i32>> {
    let mut board = vec![vec![0; ySize as usize]; xSize as usize];
    for x in 0..xSize {
        for y in 0..ySize {
            board[x as usize][y as usize] = 0;
        }
    }
    board
}

fn add_bombs(xSize: i32, ySize: i32, bombs: i32) -> Vec<Vec<i32>> {
    let mut board = empty_board(xSize, ySize);
    let mut i = 0;
    while i != bombs {
        let pos = rand::thread_rng().gen_range(0..xSize * ySize);
        if board[(pos / xSize) as usize][(pos % ySize )as usize] == 0 {
            board[(pos / xSize) as usize][(pos % ySize) as usize] = -1;
            i += 1;
        }
    }
    board
}

fn generate_board(xSize: i32, ySize: i32, bombs: i32) -> Vec<Vec<i32>> {
    add_numbers(add_bombs(xSize, ySize, bombs), xSize, ySize)
}


impl Board {
    pub fn new(width: i32, height: i32, bombs: i32) -> Board {
        Board {
            pubBoard: generate_board(width as i32, height as i32, bombs as i32),
            xSize: width as i32,
            YSize: height as i32,
            board: empty_board(width as i32, height as i32),
            bombs: bombs as i32,
        }
    }
    pub fn print(&self) {
        for x in 0..self.xSize {
            for y in 0..self.YSize {
                print!("{} ", self.pubBoard[x as usize][y as usize]);
            }
            println!("");
        }
    }


}
