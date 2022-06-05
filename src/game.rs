static EMPTY_CELL: Cell = Cell {state: false, history: 0};

#[derive(Default, Clone, Debug)]
pub struct Cell {
    state: bool,
    history: usize,
}

#[derive(Debug)]
pub struct Game {
    width: u32,
    height: u32,
    board: Vec<Vec<Cell>>,
}

impl Cell {
    fn watch_history(&self, n: u32) -> bool {
        (self.history & (2_usize).pow(n - 1)) != 0
    }

    fn change_state(&mut self, new_state: bool) {
        self.history <<= 1;
        self.history += self.state as usize;
        self.state = new_state;
    }

    fn step_back(&mut self) {
        let past_state = self.history % 2;
        self.history >>= 1;
        self.history += past_state * 2_usize.pow(
            (std::mem::size_of::<usize>() * 8 - 1).try_into().unwrap());
        self.state = past_state != 0;
    }

    pub fn get_state(&self) -> bool {
        self.state
    }
}

impl Game {
    pub fn new(width: u32, height: u32, random: bool) -> Self {
        let mut game = Self {
            width,
            height,
            board: vec![
                vec![Cell::default(); width as usize];height as usize],
        };
        if random {
            game.randomize();
        }
        game
    }

    pub fn randomize(&mut self) {
        for i in 0..self.height {
            for j in 0..self.width {
                let state = rand::random();
                self.set_cell(i as i32, j as i32, state);
            }
        }
    }

    fn count_living(&self, i: i32, j: i32) -> i32 {
        let mut living = 0;

        // for loop would be messier; the up-left cells are already
        // in a new state, so their history is used instead
        living += self.get_cell(i - 1, j - 1).watch_history(1) as i32;
        living += self.get_cell(i - 1, j + 1).watch_history(1) as i32;
        living += self.get_cell(i - 1, j).watch_history(1) as i32;
        living += self.get_cell(i, j - 1).watch_history(1) as i32;

        living += self.get_cell(i, j + 1).state as i32;
        living += self.get_cell(i + 1, j).state as i32;
        living += self.get_cell(i + 1, j - 1).state as i32;
        living += self.get_cell(i + 1, j + 1).state as i32;

        living
    }

    pub fn next_generation(&mut self) -> u32 {
        let mut alive = 0;
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                let n = self.count_living(i as i32, j as i32);
                let cell = &mut self.board[i][j];
                if cell.state {
                    let new_state = n == 2 || n == 3;
                    cell.change_state(new_state);
                    alive += new_state as u32;
                } else {
                    cell.change_state(n == 3);
                }
            }
        }
        alive
    }

    pub fn step_back(&mut self) {
        for i in 0..self.height as usize {
            for j in 0..self.width as usize {
                self.board[i][j].step_back();
            }
        }
    }

    pub fn get_cell(&self, i: i32, j: i32) -> &Cell {
        if i >= self.height as i32 || i < 0 || j < 0 || j >= self.width as i32 {
            return &EMPTY_CELL;
        }
        &self.board[i as usize][j as usize]
    }

    pub fn set_cell(&mut self, i: i32, j: i32, new_state: bool) {
        if i < self.height as i32 && i >= 0 && j >= 0 && j < self.width as i32 {
            self.board[i as usize][j as usize].change_state(new_state);
        }
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut str = format!("╭{:─^1$}╮\n", "Game of Life", (self.width as usize) * 2 + 2);
        for row in self.board[0..self.height as usize].iter() {
            str.push_str("│ ");
            for cell in row[0..self.width as usize].iter() {
                if cell.state {
                    str.push_str("██");
                } else {
                    str.push_str("  ");
                }
            }
            str.push_str(" │\n");
        }
        format!("{str}╰{:─^1$}╯\n", "", (self.width as usize) * 2 + 2)
    }
}
