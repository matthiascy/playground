#![feature(thread_id_value)]

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    sync::{Arc, Condvar, Mutex},
    thread,
    time::Duration,
};

fn main() {
    let hdl0 = thread::spawn(thread_function);
    let hdl1 = thread::spawn(thread_function);

    println!(
        "Hello from main! - thread-{id:?}",
        id = thread::current().id().as_u64()
    );

    let numbers = vec![1, 2, 3];

    // Since a thread might outlive the scope of the variables it uses,
    // Rust forces us to move the ownership of the variables to the thread.
    // This results the spawn function's arguments have a 'static lifetime.
    thread::spawn(move || {
        // move the ownership of numbers to the thread
        let id = thread::current().id().as_u64();
        println!("Hello from the thread-{id:?}!");
        for i in numbers {
            println!("  - number: {}", i);
        }
    })
    .join()
    .unwrap();

    hdl0.join().unwrap();
    hdl1.join().unwrap();

    let numbers = Vec::from_iter(0..=1000);
    let avg = thread::spawn(move || {
        let num = numbers.len() as f32;
        let sum = numbers.iter().sum::<u32>();
        sum as f32 / num
    });
    println!("Average: {}", avg.join().unwrap());

    /* Scoped Threads */
    let numbers = vec![1, 2, 3];
    thread::scope(|s| {
        s.spawn(|| {
            let count = numbers.len();
            let id = thread::current().id().as_u64();
            println!("Thread@{id:?} is counting numbers count: {count:?}");
        });
        s.spawn(|| {
            let id = thread::current().id().as_u64();
            for n in &numbers {
                println!("Thread@{id:?} is printing number: {n:?}");
            }
        });
    });

    ownership_sharing();

    mutex();

    // thread_parking();

    thread_condvar();
}

fn thread_function() {
    let id = thread::current().id().as_u64();
    println!("Hello from the thread-{id:?}!");
}

fn ownership_sharing() {
    // Share ownership by leaking.
    // Value will live to the end of the program, it will be automatically destroyed when the program exits.
    let numbers: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));
    thread::spawn(move || {
        let id = thread::current().id().as_u64();
        println!("Thread@{id:?} share ownership of numbers: {numbers:?}");
    })
    .join()
    .unwrap();
    thread::spawn(move || {
        let id = thread::current().id().as_u64();
        println!("Thread@{id:?} share ownership of numbers: {numbers:?}");
    })
    .join()
    .unwrap();

    // Share ownership using Arc.
    let array = Arc::new([1, 2, 3, 4]);
    thread::spawn({
        let array = array.clone();
        move || {
            let id = thread::current().id().as_u64();
            println!("Ownership Sharing - thread@{id:?} share ownership of array: {array:?}");
        }
    })
    .join()
    .unwrap();
    thread::spawn({
        let array = array.clone();
        move || {
            let id = thread::current().id().as_u64();
            println!("Ownership Sharing - thread@{id:?} share ownership of array: {array:?}");
        }
    })
    .join()
    .unwrap();

    thread::spawn(move || {
        let id = thread::current().id().as_u64();
        println!("Interior mutability - thread@{id:?}");
        interior_mutability();
    })
    .join()
    .unwrap();
}

fn interior_mutability() {
    // Cell - single-threaded interior mutability, does NOT allow direct borrowing of its content.
    fn cell(a: &Cell<i32>, b: &Cell<i32>, v: &Cell<Vec<i32>>) {
        let before = a.get();
        b.set(b.get() + 1);
        let after = a.get();
        if before != after {
            println!("Cell: a changed from {} to {}", before, after);
        }

        let mut arr = v.take();
        arr.push(4);
        println!("Cell: arr = {arr:?}");
        v.set(arr);
    }
    let arr = Cell::new(vec![1, 2, 3]);
    let val = Cell::new(1);
    cell(&val, &val, &arr);
    println!("Cell: val = {}", val.get());

    // RefCell - single-threaded interior mutability, allows direct borrowing of its content with runtime cost.
    let arr = RefCell::new(vec![1, 2, 3]);
    arr.borrow_mut().push(4);
    println!("RefCell: arr = {arr:?}");

    // RwLock - Concurrent version of RefCell.
}

fn mutex() {
    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut n = n.lock().unwrap();
                for _ in 0..100 {
                    *n += 1;
                }
            });
        }
    });
    println!("Mutex updaten n = {:?}", n.into_inner().unwrap());
}

fn thread_parking() {
    let queue = Mutex::new(VecDeque::new());
    thread::scope(|s| {
        // Consumer
        let consumer = s.spawn(|| loop {
            let item = queue.lock().unwrap().pop_front();
            if let Some(item) = item {
                println!("Consume: {item:?}");
            } else {
                println!("Queue is empty ==> parking");
                thread::park();
            }
        });

        // Producer
        for i in 0.. {
            println!("Produce: {i:?}");
            queue.lock().unwrap().push_back(i);
            consumer.thread().unpark();
            thread::sleep(Duration::from_secs(1));
        }
    });
}

fn thread_condvar() {
    let queue = Mutex::new(VecDeque::new());
    let not_empty = Condvar::new();

    thread::scope(|s| {
        s.spawn(|| loop {
            let mut q = queue.lock().unwrap();
            let item = loop {
                if let Some(item) = q.pop_front() {
                    break item;
                } else {
                    // Unlocking, waiting, and relocking is all done by the wait() method.
                    // The wait() method takes a MutexGuard and returns a new MutexGuard.
                    println!("Queue is empty ==> waiting");
                    q = not_empty.wait(q).unwrap();
                }
            };
            drop(q); // Explicitly drop the guard to release the lock before consuming the item.
            println!("Consume: {item:?}");
        });

        for i in 0.. {
            println!("Produce: {i:?}");
            queue.lock().unwrap().push_back(i);
            not_empty.notify_one();
            thread::sleep(Duration::from_secs(1));
        }
    });
}
