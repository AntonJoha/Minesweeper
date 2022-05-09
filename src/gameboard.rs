use rand::Rng;

pub enum BoardState {
    Playing,
    Win,
    Loss,
}

pub struct Board {
    pubBoard: Vec<Vec<i32>>,
    xSize: i32,
    YSize: i32,
    board: Vec<Vec<i32>>,
    bombs: i32,
    state: BoardState,
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

fn init_board(xSize: i32, ySize: i32, value: i32) -> Vec<Vec<i32>> {
    let mut board = vec![vec![0; ySize as usize]; xSize as usize];
    for x in 0..xSize {
        for y in 0..ySize {
            board[x as usize][y as usize] = value;
        }
    }
    board
}

fn add_bombs(xSize: i32, ySize: i32, bombs: i32) -> Vec<Vec<i32>> {
    let mut board = init_board(xSize, ySize, 0);
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
            board: generate_board(width as i32, height as i32, bombs as i32),
            xSize: width as i32,
            YSize: height as i32,
            pubBoard: init_board(width as i32, height as i32, 10 as i32),
            bombs: bombs as i32,
            state: BoardState::Playing,
        }
    }
    pub fn print(&self) {
        for x in 0..self.xSize {
            for y in 0..self.YSize {
                print!("{}\t", self.pubBoard[x as usize][y as usize]);
            }
            println!("");
        }
    }
    
    pub fn get_board(&self) -> Vec<Vec<i32>> {
        self.pubBoard.clone()
    }
    
    pub fn guess(&mut self, x: i32, y :i32) -> bool { return self.guess_aux(x, y, true); }

    pub fn guess_aux(&mut self, x: i32, y: i32, first: bool) -> bool {
        if self.board[x as usize][y as usize] == -1 {
            if first {
                self.state = BoardState::Loss;
                self.pubBoard[x as usize][y as usize] = -1;
            }
            false
        }
        else if self.pubBoard[x as usize][y as usize] != 10 {
            true
        }
        else 
        {
            self.pubBoard[x as usize][y as usize] = self.board[x as usize][y as usize];
            if self.board[x as usize][y as usize] == 0 {
                let offset = vec![-1, 0, 1];
            
                for i in 0..3 {
                    for j in 0..3 {
                        if !(i == 0 && j == 0) && 
                            !self.out_of_bounds(x + offset[i], y + offset[j]){
                                self.guess_aux(x + offset[i], y + offset[j], false);
                        }
                    }
                }
            }
            true
        }
    }
    
    pub fn get_state(&self) -> BoardState {
        self.state.clone()
    }

    fn out_of_bounds(&self, x: i32, y: i32) -> bool {
        x < 0 || x >= self.xSize || y < 0 || y >= self.YSize
    }
}


