use std::cell::RefCell;
use std::rc::{Rc, Weak};

struct Cell {
    state: bool,
    neighbors: Option<Vec<Weak<RefCell<Cell>>>>,
}

fn main() {
    let mut world = vec![];
    for _ in 0..10 {
        let cell = Cell {
            state: false,
            neighbors: None,
        };
        world.push(Rc::new(RefCell::new(cell)));
    }

    for i in 0..world.len() {
        let mut cell = world[i].borrow_mut();
        cell.neighbors = Some(vec![
            Rc::downgrade(&world[(i + 1) % world.len()]),
            Rc::downgrade(&world[(i + 2) % world.len()]),
            Rc::downgrade(&world[(i + world.len() - 1) % world.len()]),
            Rc::downgrade(&world[(i + world.len() - 2) % world.len()]),
        ]);
    }

    for cell in &world {
        let score = cell
            .borrow()
            .neighbors
            .as_deref()
            .unwrap_or(&[])
            .iter()
            .filter_map(|weak| weak.upgrade())
            .filter(|rc| rc.borrow().state)
            .count();
        println!("Score: {}", score);
    }

    println!("Hello, world!");
}
