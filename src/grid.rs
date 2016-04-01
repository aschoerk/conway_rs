use cell::Cell;
use seeds::Seed;

pub struct Grid {
    pub cells: Vec<Cell>,
    pub checksum: u64,
}

impl Grid {
    pub fn new(seed: Seed, width: i16, height: i16, square_size: f32) -> Grid {
        let mut cells = Vec::new();
        
        let mut checksum = 0u64;
        let mut current_bit = 1u64;

        for y in 0..height {
            for x in 0..width {
                let alive = seed(x, y); 
                cells.push(Cell {
                    x: (x as f32 * square_size + square_size / 2.),
                    y: -(y as f32 * square_size + square_size / 2.),
                    scale: square_size,
                    neighbours: [
                        (x-1, y-1), (x, y-1), (x+1, y-1),
                        (x-1, y  ),           (x+1, y  ),
                        (x-1, y+1), (x, y+1), (x+1, y+1)
                    ].iter().map(|n| coords_to_index(*n, width, height)).collect(),
                    alive: alive,
                });
                
                if alive {
                    checksum ^= current_bit;
                }
                current_bit <<= 1;
                if current_bit == 0 {
                    current_bit = 1;
                }
            }
        }
        Grid { cells: cells, checksum: checksum }
    }

    pub fn update(&mut self) {
        let mut checksum = 0u64;
        let mut current_bit = 1u64;
        
        let mut alive_neighbours = Vec::new();
        for cell in self.cells.iter() {
            alive_neighbours.push(cell.neighbours.iter().filter(|n| self.cells[**n].alive).count())
        }

        for (cell, cell_alive_neighbours) in self.cells.iter_mut().zip(alive_neighbours.iter()) {
            cell.update(*cell_alive_neighbours);
            if cell.alive {
                checksum ^= current_bit;
            }
            current_bit <<= 1;
            if current_bit == 0 {
                current_bit = 1;
            }
        }
        self.checksum = checksum;
    }
}

fn coords_to_index(coords: (i16, i16), grid_width: i16, grid_height: i16) -> usize {
    let x: i32 = coords.0 as i32;
    let y: i32 = coords.1 as i32;
    let x_wrapped = (x + grid_width as i32) % grid_width as i32;
    let y_wrapped = (y + grid_height as i32) % grid_height as i32;
    (x_wrapped + (y_wrapped * grid_width as i32)) as usize
}

#[cfg(test)]
mod tests {
    use super::coords_to_index;

    #[test]
    fn it_returns_the_x_value_on_the_first_row() {
        assert!(coords_to_index((3, 0), 5, 3) == 3)
    }

    #[test]
    fn it_wraps_overflowing_x_values() {
        assert!(coords_to_index((6, 0), 5, 3) == 1)
    }

    #[test]
    fn it_wraps_underflowing_x_values() {
        assert!(coords_to_index((-1, 0), 5, 3) == 4)
    }

    #[test]
    fn it_adds_one_width_for_each_row() {
        assert!(coords_to_index((2, 2), 5, 3) == 12)
    }

    #[test]
    fn it_wraps_overflowing_y_values() {
        assert!(coords_to_index((1, 5), 5, 3) == 11)
    }

    #[test]
    fn it_wraps_underflowing_y_values() {
        assert!(coords_to_index((4, -2), 5, 3) == 9)
    }
}
