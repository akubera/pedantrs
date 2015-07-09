#![feature(plugin)]
#![plugin(pedantrs)]

use std::thread;
use std::sync::{Arc, Mutex};

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

struct Table {
    forks: Vec<Mutex<()>>,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Philosopher {
        Philosopher {
            name: name.to_string(),
            left: left,
            right: right,
        }
    }

    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating", self.name);
        thread::sleep_ms(1000);
        println!("{} is done eating", self.name);
    }
}

fn lots_of_args(_: i32, _: i32, _: i32, _: i32, _:i32, _: i32, _: i32) {
}

fn main() {
    let table = Arc::new(Table { forks: vec![
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
        Mutex::new(()),
    ]});

    let philosophers = vec![
        Philosopher::new("p1", 0, 1),
        Philosopher::new("p2", 1, 2),
        Philosopher::new("p3", 2, 3),
        Philosopher::new("p4", 3, 4),
        Philosopher::new("p5", 4, 0)
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        let table = table.clone();

        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }

    lots_of_args(1, 2, 3, 4, 5, 6, 7);
}
