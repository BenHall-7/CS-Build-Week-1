#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use ndarray::{Array2, Shape};

/// An instance of Conway's Game of Life.
pub struct Game {
    /// The double buffer used to update the game state.
    /// When one grid is used to view the game state, the other is used to update before their purposes switch.
    grid: [Array2<u8>; 2],
    /// The number grid used for viewing
    front: u8,
    /// Determines if the top-and-bottom and left-and-right borders are considered connected
    repeating: bool,
}

impl Game {
    pub fn new(width: usize, height: usize, repeating: bool) -> Self {
        let array0 = Array2::zeros((width, height));
        let array1 = Array2::zeros((width, height));
        Game {
            grid: [array0, array1],
            front: 0,
            repeating
        }
    }

    pub fn set(&mut self, alive: Vec<[usize; 2]>) {
        for coord in alive {
            self.grid[0][coord] = 1;
        }
    }

    fn front_mut<'a>(&'a mut self) -> &'a mut Array2<u8> {
        &mut self.grid[self.front as usize]
    }

    fn back_mut<'a>(&'a mut self) -> &mut Array2<u8> {
        &mut self.grid[1 - self.front as usize]
    }

    fn front<'a>(&'a self) -> &'a Array2<u8> {
        &self.grid[self.front as usize]
    }
    
    fn back<'a>(&'a self) -> &'a Array2<u8> {
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
        let (x, y) = (coord[0], coord[1]);
        // representing if we have a neighbor in X direction:
        let left = x != 0;
        let right = x != front.ncols() - 1;
        let top = y != 0;
        let bottom = y != front.nrows() - 1;

        macro_rules! check_coord {
            ($x:expr, $y:expr) => {
                if front[[$x, $y]] != 0 { neighbors += 1; }
            }
        }
        
        if left { check_coord!(x - 1, y) }
        if left && top { check_coord!(x - 1, y - 1) }
        if top { check_coord!(x, y - 1) }
        if top && right { check_coord!(x + 1, y - 1) }
        if right { check_coord!(x + 1, y) }
        if right && bottom { check_coord!(x + 1, y + 1) }
        if bottom { check_coord!(x, y + 1) }
        if bottom && left { check_coord!(x - 1, y + 1) }

        self.back_mut()[coord] = Self::alive(front[coord] != 0, neighbors) as u8;
    }

    pub fn step(&mut self) {
        let cols = self.back().ncols();
        let rows = self.back().nrows();
        for x in 0..cols {
            for y in 0..rows {
                self.update_cell([x ,y]);
            }
        }
    }
}