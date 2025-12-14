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

fn main() {
    let mut world = vec![];
    let mut rng = rand::rng();

    for _ in 0..10 {
        let cell = Cell {
            state: rng.random(),
            neighbors: None,
        };
        world.push(Rc::new(RefCell::new(cell)));
    }

    for i in 0..world.len() {
        let mut cell = world[i].borrow_mut();
        cell.neighbors = Some(vec![
            Rc::downgrade(&world[if i < 5 { (i + 1) % 5 } else { 5 + (i - 5 + 1) % 5 }]),
            Rc::downgrade(&world[if i < 5 { (i + 4) % 5 } else { 5 + (i - 5 + 4) % 5 }]),
            Rc::downgrade(&world[(i + 5) % world.len()]),
            Rc::downgrade(&world[if i < 5 { 5 + (i + 1) % 5 } else { (i - 5 + 4) % 5 }]),
        ]);
        println!("cell neighbors indices for {:?}: {:?} {:?} {:?} {:?}", i, if i < 5 { (i + 1) % 5 } else { 5 + (i - 5 + 1) % 5 }, if i < 5 { (i + 4) % 5 } else { 5 + (i - 5 + 4) % 5 }, (i + 5) % world.len(), if i < 5 { 5 + (i + 1) % 5 } else { (i - 5 + 4) % 5 });
    }

    while world.iter().any(|cell| cell.borrow().state) {
        let mut next = vec![];
        for cell in &world {
            let score = cell
                .borrow()
                .neighbors
                .as_deref()
                .unwrap_or(&[])
                .iter()
                .filter_map(|weak| weak.upgrade())
                .filter(|rc| rc.borrow().state)
                .count() + if cell.borrow().state { 1 } else { 0 };
            next.push(score == 3 || (score == 2 && cell.borrow().state));
        }

        for i in 0..world.len() {
            let mut cell = world[i].borrow_mut();
            cell.state = next[i];
        }

        let nh_raw = world.iter().take(5).map(|c| c.borrow().state);
        let sh_raw = world.iter().skip(5).map(|c| c.borrow().state);

        let row1 = nh_raw
            .clone()
            .map(|s| if s { '▲' } else { '△' });

        let row3 = sh_raw
            .clone()
            .map(|s| if s { '▼' } else { '▽' });

        let row2 = sh_raw.clone().map(|s| if s { '▲' } else { '△' })
            .zip(nh_raw.map(|s| if s { '▼' } else { '▽' }))
            .flat_map(|(a, b)| [a, b]);

        let nh = world.iter().take(5).map(|c| if c.borrow().state { '█' } else { ' ' }).collect::<String>();
        let sh = world.iter().skip(5).map(|c| if c.borrow().state { '█' } else { ' ' }).collect::<String>();

        println!("{}", row1.map(|c| format!(" {}", c)).collect::<String>());
        println!("{}", row2.collect::<String>());
        print!("{}", row3.map(|c| format!("{} ", c)).collect::<String>());

        print!("\x1B[2A\r");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(500));
    }

    // println!("simulation ended!");

    // println!(" ▲ ▲ ▲ ▲ ▲");
    // println!("▲▼▲▼▲▼▲▼▲▼");
    // println!("▼ ▼ ▼ ▼ ▼");
}
