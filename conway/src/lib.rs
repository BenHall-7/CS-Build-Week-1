#![allow(dead_code)]

use ndarray::Array2;

/// An instance of Conway's Game of Life.
pub struct Game {
    /// The double buffer used to update the game state.
    /// When one grid is used to view the game state, the other is used to update before their purposes switch.
    grid: [Array2<u8>; 2],
    /// The number grid used for viewing
    front: u8,
    /// Determines if the top-and-bottom and left-and-right borders are considered connected
    repeating: bool,
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize, repeating: bool) -> Self {
        let array0 = Array2::zeros((height, width));
        let array1 = Array2::zeros((height, width));
        Game {
            grid: [array0, array1],
            front: 0,
            repeating,
            width,
            height
        }
    }

    pub fn set_on<I: IntoIterator<Item=[usize; 2]>>(&mut self, cells: I) {
        for coord in cells {
            self.grid[self.front as usize][coord] = 1;
        }
    }

    pub fn set_off<I: IntoIterator<Item=[usize; 2]>>(&mut self, cells: I) {
        for coord in cells {
            self.grid[self.front as usize][coord] = 0;
        }
    }

    pub fn invert<I: IntoIterator<Item=[usize; 2]>>(&mut self, cells: I) {
        for coord in cells {
            let current = &mut self.grid[self.front as usize][coord];
            if *current == 0 {
                *current = 1;
            } else {
                *current = 0;
            }
        }
    }

    pub fn clear(&mut self) {
        *self = Game::new(self.width, self.height, self.repeating)
    }

    // TODO:
    // pub fn resize(&mut self, width: usize, height: usize) {

    // }

    pub fn back_mut(&mut self) -> &mut Array2<u8> {
        &mut self.grid[1 - self.front as usize]
    }

    pub fn front(&self) -> &Array2<u8> {
        &self.grid[self.front as usize]
    }
    
    pub fn back(&self) -> &Array2<u8> {
        &self.grid[1 - self.front as usize]
    }

    /// The basic rule to Conway's version of the automata for cells
    fn alive(already_alive: bool, neighbors: usize) -> bool {
        match (already_alive, neighbors) {
            (_, 3) | (true, 2) => true,
            _ => false
        }
    }

    fn update_cell(&mut self, coord: [usize; 2]) {
        let front = self.front();

        // TODO: handle repeating case
        
        let mut neighbors = 0;
        let (y, x) = (coord[0], coord[1]);
        // representing if we have a neighbor in X direction:
        let left = x > 0;
        let right = x < front.ncols() - 1;
        let top = y > 0;
        let bottom = y < front.nrows() - 1;

        macro_rules! check_coord {
            ($y:expr, $x:expr) => {
                if front[[$y, $x]] != 0 { neighbors += 1; }
            }
        }
        
        if left { check_coord!(y, x - 1) }
        if left && top { check_coord!(y - 1, x - 1) }
        if top { check_coord!(y - 1, x) }
        if top && right { check_coord!(y - 1, x + 1) }
        if right { check_coord!(y, x + 1) }
        if right && bottom { check_coord!(y + 1, x + 1) }
        if bottom { check_coord!(y + 1, x) }
        if bottom && left { check_coord!(y + 1, x - 1) }

        self.back_mut()[coord] = Self::alive(front[coord] != 0, neighbors) as u8;
    }

    pub fn step(&mut self) {
        let cols = self.back().ncols();
        let rows = self.back().nrows();
        for x in 0..cols {
            for y in 0..rows {
                self.update_cell([y, x]);
            }
        }
        self.front = 1 - self.front;
    }
}

#[test]
fn test_blinker() {
    let mut game = Game::new(3, 3, false);
    game.set_on(vec![[0, 1], [1, 1], [2, 1]]);
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 0, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([1, 1, 1].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([0, 0, 0].as_ref()));
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 1, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([0, 1, 0].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([0, 1, 0].as_ref()));
}

#[test]
fn test_glider() {
    let mut game = Game::new(4, 4, false);
    // 0 1 0 0
    // 0 0 1 0
    // 1 1 1 0
    // 0 0 0 0
    game.set_on(vec![[0, 1], [1, 2], [2, 0], [2, 1], [2, 2]]);
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 0, 0, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([1, 0, 1, 0].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([0, 1, 1, 0].as_ref()));
    assert_eq!(game.front().row(3).to_slice(), Some([0, 1, 0, 0].as_ref()));
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 0, 0, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([0, 0, 1, 0].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([1, 0, 1, 0].as_ref()));
    assert_eq!(game.front().row(3).to_slice(), Some([0, 1, 1, 0].as_ref()));
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 0, 0, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([0, 1, 0, 0].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([0, 0, 1, 1].as_ref()));
    assert_eq!(game.front().row(3).to_slice(), Some([0, 1, 1, 0].as_ref()));
    game.step();
    assert_eq!(game.front().row(0).to_slice(), Some([0, 0, 0, 0].as_ref()));
    assert_eq!(game.front().row(1).to_slice(), Some([0, 0, 1, 0].as_ref()));
    assert_eq!(game.front().row(2).to_slice(), Some([0, 0, 0, 1].as_ref()));
    assert_eq!(game.front().row(3).to_slice(), Some([0, 1, 1, 1].as_ref()));
}
