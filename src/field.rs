#[derive(Clone)]
enum CellState {
    Alive,
    Dead,
    Border,
}
pub struct Field {
    cells: Vec<Vec<CellState>>
}

impl Field {
    pub fn init(size: usize) -> Self {
        let size_with_borders = size + 2;
        let last_index = size_with_borders - 1;
        let mut row_with_borders = vec![CellState::Border; 1];
        let mut row = vec![CellState::Dead; size];
        row_with_borders.append(&mut row);
        row_with_borders.push(CellState::Border);
        let mut cells: Vec<Vec<CellState>> = vec![];
        for index in 0..size_with_borders {
            match index {
                0 => {
                    cells.push(vec![CellState::Border; size_with_borders]);
                },
                _ if index == last_index => {
                    cells.push(vec![CellState::Border; size_with_borders]);
                },
                _ => {
                    let unique_row = row_with_borders.to_vec();
                    cells.push(unique_row);
                },
            }
        }

        Field { cells }
    }

    pub fn print(&mut self) {
        for row in &self.cells {
            for cell in row {
                match cell {
                    CellState::Alive => {
                        print!("{}", '■');
                    },
                    CellState::Dead => {
                        print!("{}", '.');
                    },
                    CellState::Border => {
                        print!("{}", '/');
                    },
                }
            }
            print!("{}", '\n');
        }
    }

    pub fn make_alive(&mut self, row: usize, column: usize) {
        self.cells[row][column] = CellState::Alive;
    }

    pub fn make_dead(&mut self, row: usize, column: usize) {
        self.cells[row][column] = CellState::Dead;
    }

    pub fn step(&mut self) {
        let size_with_borders = self.cells.len();
        let mut copy_cells = self.cells.to_vec();
        for row in 1..size_with_borders - 1 {
            for column in 1..size_with_borders - 1 {
                let alive_neighbours = self.count_alive_around(row, column);
                
                match self.cells[row][column] {
                    CellState::Alive => {
                        match alive_neighbours {
                            2 | 3 => {},
                            _ => { copy_cells[row][column] = CellState::Dead; },
                        }
                    },
                    CellState::Dead => {
                        match alive_neighbours {
                            3 => { copy_cells[row][column] = CellState::Alive; },
                            _ => {},
                        }
                    },
                    CellState::Border => {},
                }
            }
        }
        self.cells = copy_cells;

    }

    pub fn count_alive_around(&mut self, cell_row: usize, cell_column: usize) -> u32 {
        let mut counter: u32 = 0;
        let size = self.cells.len() - 2;
        for row in cell_row - 1..=cell_row + 1 {
            for column in cell_column - 1..=cell_column + 1 {
                if cell_row == row && cell_column == column {
                    continue;
                }
                match self.cells[row][column] {
                    CellState::Alive => {
                        counter += 1;
                    },
                    CellState::Dead => {},
                    CellState::Border => {
                        let mut searched_row = row;
                        let mut searched_column = column;

                        if row == 0 {
                            searched_row += size;
                        }
                        else if row == size + 1 {
                            searched_row -= size;
                        }

                        if column == 0 {
                            searched_column += size;
                        }
                        else if column == size + 1 {
                            searched_column -= size;
                        }

                        match self.cells[searched_row][searched_column] {
                            CellState::Alive => { 
                                counter += 1;
                            },
                            CellState::Dead => {},
                            CellState::Border => {},
                        }
                    },
                }
            }
        }
        //println!("Number of neighbours at {:?}-{:?} is {:?}", cell_row, cell_column, counter);
        counter
    }
}