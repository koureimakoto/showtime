mod utils;
use wasm_bindgen::prelude::*;
use std::fmt;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum 
Cell {
    Dead = 0,
    Alive= 1
}

#[wasm_bindgen]
pub struct 
Universe {
    w: u32,
    h: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl 
Universe {
    pub fn
    new() -> Universe {
        let w = 64;
        let h = 64;

        let cells = ( 0..w * h ).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe { w, h, cells }

    }

    pub fn
    render( &self ) -> String {
        self.to_string()
    }


    pub fn
    tick( &mut self ) {
        let mut next = self.cells.clone();

        for row in 0..self.h {
            for col in 0..self.w {
                let id  : usize = self.get_index(row, col);
                let cell: Cell  = self.cells[id];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    ( Cell::Alive, x ) if x < 2 => Cell::Dead,
                    ( Cell::Alive, 2 ) | ( Cell::Alive, 3 ) => Cell::Alive,
                    ( Cell::Alive, x ) if x > 3 => Cell::Dead,
                    ( Cell::Dead , 3 ) => Cell::Alive,
                    ( otherwise, _   ) => otherwise, 
                };

                next[id] = next_cell;
            }
        }
        self.cells = next;
    }


    fn 
    get_index( &self, row: u32, column: u32 ) -> usize {
        ( row * self.w + column ) as usize
    }

    fn 
    live_neighbor_count( &self, row: u32, column: u32 ) -> u8 {
        
        let mut
            count = 0;

        for delta_row in [ self.h - 1, 0, 1].iter().cloned() {
            for delta_col in [ self.w -1, 0, 1].iter().cloned() {

                if delta_row == 0 && delta_col == 0 {
                    continue;
                }

                let neighbor_row: u32 = ( row + delta_row ) % self.h;
                let neighbor_col: u32 = ( column + delta_col ) % self.w;

                let id = self.get_index( neighbor_row, neighbor_col );

                count += self.cells[id] as u8;

            }
        }
        count
    }

}


impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.w as usize ) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!( f, "{}", symbol)?;
            }
            write!( f, "\n")?;
        }
        Ok(())
    }
}



#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert( s: &str );
}

#[wasm_bindgen]
pub fn
greet( name: &str ) {
    alert( &format!("Hello {}, to the Game of Life", name) );
}