use std::thread;
use std::sync::{Mutex, Arc};

struct Table {
    forks: Vec<Mutex<()>>,
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
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
        // The call to lock() might fail, and if it does, we want to crash. In this case, the error that could happen is that the mutex is ‘poisoned’, which is what happens when the thread panics while the lock is held. Since this shouldn’t happen, we just use unwrap().
        //
        // What’s up with that underscore? Well, we aren’t planning on using the value inside the lock. We just want to acquire it. As such, Rust will warn us that we never use the value. By using the underscore, we tell Rust that this is what we intended, and it won’t throw a warning.
        //
        // What about releasing the lock? Well, that will happen when _left and _right go out of scope, automatically.
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();
        println!("{} is eating", self.name);

        thread::sleep_ms(1000);

        println!("{} is done eating", self.name);
    }
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
        Philosopher::new("Judith Butler", 0, 1),
        Philosopher::new("Gilles Deleuze", 1, 2),
        Philosopher::new("Karl Marx", 2, 3),
        Philosopher::new("Emma Goldman", 3, 4),
        // Monsieur Foucault should have 4, 0 as arguments, but instead, has 0, 4. This is what prevents deadlock, actually: one of our philosophers is left handed! This is one way to solve the problem
        Philosopher::new("Michel Foucault", 0, 4),
    ];

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        // The clone() method on Arc<T> is what bumps up the reference count, and when it goes out of scope, it decrements the count. This is needed so that we know how many references to table exist across our threads. If we didn’t have a count, we wouldn’t know how to deallocate it.
        let table = table.clone();

        // "move" indicates that the closure is going to take ownership of the values it’s capturing. Primarily, the p variable of the map function.
        thread::spawn(move || {
            p.eat(&table);
        })
    }).collect();

    for h in handles {
        // join() blocks execution until the thread has completed execution. This ensures that the threads complete their work before the program exits.
        h.join().unwrap();
    }
}
