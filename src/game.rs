mod gameboard;

struct Game {
    board: gameboard::GameBoard,
}


pub fn start_game() {
    let mut game = Game::new();

}


fn get_bomb_neighbors(x: i32, y :i32, &table: Vec<Vec<i32>>) -> i32 {
    
    let offset = vec![-1, 0, 1];
    let mut bomb_neighbors = 0;

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }
            if ( x + offset[i]) < 0 || ( x + offset[i]) > table.len() - 1 {
                continue;
            }
            if ( y + offset[j]) < 0 || ( y + offset[j]) > table[0].len() - 1 {
                continue;
            }
            if table[(x + offset[i]) as usize][(y + offset[j]) as usize] == -1 {
                bomb_neighbors += 1;
            }
        }
    }
    bomb_neighbors
}


fn get_empty_neighbors_pos( x: i32, y: i32, mut & table: vec<vec<i32>>) -> Vec<(i32, i32)> {

    let offset = vec![-1, 0, 1];
    let mut empty_neighbors = Vec::new();

    for i in 0..3 {
        for j in 0..3 {
            if x + offset[i] < xSize && y + offset[j] < ySize  && !(i == 1 && j == 1) {
                if (x + offset[i] >= 0 && y + offset[j] >= 0) {
                    if table[x + offset[i]][y + offset[j]] == 10 {
                        empty_neighbors.push((x + offset[i], y + offset[j]));
                    }
                }
            }
        }
    }
    return empty_neighbors;

}



fn get_empty_neighbors( x: i32, y: i32, mut & table: vec<vec<i32>>) -> i32 {

    let offset = vec![-1, 0, 1];
    let mut count = 0;

    for i in 0..3 {
        for j in 0..3 {
            if x + offset[i] < xSize && y + offset[j] < ySize  && !(i == 1 && j == 1) {
                if (x + offset[i] >= 0 && y + offset[j] >= 0) {
                    if table[x + offset[i]][y + offset[j]] == 10 {
                        count += 1;
                    }
                }
            }
        }
    }
    return count;
}

fn add_bombs(x: i32, y: i32, mut & table: Vec<Vec<i32>>) {
    let offset = vec![-1, 0, 1];
 
    for i in 0..3 {
        for j in 0..3 {
            if x + offset[i] < xSize && y + offset[j] < ySize  && !(i == 1 && j == 1) {
                if (x + offset[i] >= 0 && y + offset[j] >= 0) {
                    if table[x + offset[i]][y + offset[j]] == 10 {
                        table[x + offset[i]][y + offset[j]] = -1;
                    }
                }
            }
        }
    }
}

fn mark_bombs(table: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut table = table;
    let xSize = table.len();
    let ySize = table[0].len();

    let mut change = true;

    while (change) {
        change = false;
        for x in 0..xSize {
            for y in 0..ySize {
                if table[x][y] == 10 || table[x][y] == 0 {
                    continue;
                }
                
                let empty_neighbors = get_empty_neighbors(x, y, table);
                let bomb_neighbors = get_bomb_neighbors(x, y, table);
                if empty_neighbors + bomb_neighbors  == table[x][y] && table[x][y] != bomb_neighbors {
                    add_bombs(x, y, table);
                    change = true;
                }
            }
        }
    }
    table
}

fn find_obvious_moves(table: & Vec<Vec<i32>>)) -> Vec<(i32, i32)>{
    let xSize = table.len();
    let ySize = table[0].len();

    let mut moves = Vec::new();

    for x in 0..xSize {
        for y in 0..ySize {
            let empty_neighbors = get_empty_neighbors(x, y, table);
            let bomb_neighbors = get_bomb_neighbors(x, y, table);
            if bomb_neighbors == table[x][y] && empty_neighbors != 0{
                let list = get_empty_neighbors_pos(x, y, table);
                for i in 0..list.len() {
                    moves.push(list[i]);
                }
            }
        }
    }
}

impl Game{

    pub fn new() -> Game {
        Game {
            board: gameboard::GameBoard::new(),
        }
    }

    pub fn get_gamestate(&self) -> &gameboard::GameBoard {
        &self.board.get_gamestate()
    }

    
    pub fn solve(&mut self) {
        while self.get_gamestate() == BoardState::Playing {
            self.next_move();
        }
    }

    fn next_move(&mut self) {
        let table : Vec<Vec<i32>> = mark_bombs(board.get_board());
        let pos : Vec<i32, i32> = find_obvious_moves(table);
        if pos.len() != 0 {
            for i in 0..pos.len() {
                self.board.click(pos[i].0, pos[i].1);
            }
        } else {

        }
    }

    

}


