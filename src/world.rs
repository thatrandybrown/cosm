use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::{Rc, Weak};

use rand::prelude::*;
use std::thread;
use std::time::Duration;

struct Cell {
    state: bool,
    neighbors: Option<Vec<Weak<RefCell<Cell>>>>,
}

pub struct World {
    cells: Vec<Rc<RefCell<Cell>>>,
}

impl World {
    pub fn new() -> Self {
        let mut cells = Vec::new();
        for _ in 0..10 {
            cells.push(Rc::new(RefCell::new(Cell {
                state: rand::random(),
                neighbors: None,
            })));
        }

        for i in 0..cells.len() {
            let mut cell = cells[i].borrow_mut();
            cell.neighbors = Some(vec![
                Rc::downgrade(
                    &cells[if i < 5 {
                        (i + 1) % 5
                    } else {
                        5 + (i - 5 + 1) % 5
                    }],
                ),
                Rc::downgrade(
                    &cells[if i < 5 {
                        (i + 4) % 5
                    } else {
                        5 + (i - 5 + 4) % 5
                    }],
                ),
                Rc::downgrade(&cells[(i + 5) % cells.len()]),
                Rc::downgrade(
                    &cells[if i < 5 {
                        5 + (i + 1) % 5
                    } else {
                        (i - 5 + 4) % 5
                    }],
                ),
            ]);
        }

        World { cells }
    }

    pub fn next(&mut self) -> World {
        let mut cells = Vec::new();

        for cell in &self.cells {
            let score = cell
                .borrow()
                .neighbors
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .filter_map(|weak| weak.upgrade())
                .filter(|rc| rc.borrow().state)
                .count()
                + if cell.borrow().state { 1 } else { 0 };
            cells.push(Rc::new(RefCell::new(Cell {
                state: score == 3 || (score == 2 && cell.borrow().state),
                neighbors: cell.borrow().neighbors.clone(),
            })));
        }

        World { cells }
    }

    pub fn cells(&self) -> Vec<bool> {
        self.cells.iter().map(|cell| cell.borrow().state).collect()
    }
}
