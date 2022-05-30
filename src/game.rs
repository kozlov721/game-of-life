use rand::Rng;

#[derive(Default, Clone, Debug)]
struct Cell {
    state: bool,
    history: u32,
}

#[derive(Debug)]
pub struct Game {
    width: usize,
    height: usize,
    board: Vec<Vec<Cell>>,
    alive: u32,
}

impl Cell {
    fn watch_history(&self, n: u32) -> bool {
        (self.history & (2_u32).pow(n - 1)) != 0
    }

    fn change_state(&mut self, new_state: bool) {
        self.history <<= 1;
        self.history += self.state as u32;
        self.state = new_state;
    }

    fn step_back(&mut self) {
        let past_state = self.history % 2;
        self.history >>= 1;
        self.history += past_state * 2_u32.pow(32);
        self.state = past_state != 0;
    }
}

impl Game {
    pub fn new(width: usize, height: usize, random: bool) -> Self {
        let mut game = Self {
            width,
            height,
            // two more rows and columns as const 0 boundaries
            board: vec![vec![Cell::default(); width + 2]; height + 2],
            alive: 0,
        };
        if random {
            game.randomize();
        }
        game
    }

    pub fn randomize(&mut self) {
        let mut rng = rand::thread_rng();
        self.alive = 0;
        for i in 1..self.height - 1 {
            for j in 1..self.width - 1 {
                let cell = &mut self.board[i][j];
                cell.state = rng.gen_range(0..=1) != 0;
                cell.history = 0;
                self.alive += cell.state as u32;
            }
        }
    }

    fn count_living(&self, i: usize, j: usize) -> i32 {
        let mut living = 0;

        // for loop would be messier; the up-left cells are already
        // in a new state, so their history is used instead
        living += self.board[i - 1][j - 1].watch_history(1) as i32;
        living += self.board[i - 1][j + 1].watch_history(1) as i32;
        living += self.board[i - 1][j].watch_history(1) as i32;
        living += self.board[i][j - 1].watch_history(1) as i32;

        living += self.board[i][j + 1].state as i32;
        living += self.board[i + 1][j].state as i32;
        living += self.board[i + 1][j - 1].state as i32;
        living += self.board[i + 1][j + 1].state as i32;

        living
    }

    pub fn next_generation(&mut self) {
        for i in 1..self.height - 1 {
            for j in 1..self.width - 1 {
                let n = self.count_living(i, j);
                let cell = &mut self.board[i][j];
                if cell.state {
                    cell.change_state(n == 2 || n == 3);
                } else {
                    cell.change_state(n == 3);
                }
            }
        }
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        let mut str = format!("╭{:─^1$}╮\n", "Game of Life", self.width * 2 - 2);
        for row in self.board[1..self.height - 1].iter() {
            str.push_str("│ ");
            for cell in row[1..self.width - 1].iter() {
                if cell.state {
                    str.push_str("██");
                } else {
                    str.push_str("  ");
                }
            }
            str.push_str(" │\n");
        }
        format!("{str}╰{:─^1$}╯\n", "", self.width * 2 - 2)
    }
}
