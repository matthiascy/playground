#![feature(thread_id_value)]

use std::collections::HashMap;
use std::sync::atomic::{AtomicPtr, Ordering};

#[derive(Debug)]
struct Data {
    a: HashMap<u32, u32>,
    b: HashMap<u32, u32>,
}

impl Default for Data {
    fn default() -> Self {
        let mut a = HashMap::new();
        a.insert(1, 2);
        let mut b = HashMap::new();
        b.insert(3, 4);
        Self { a, b }
    }
}

fn get_data() -> &'static Data {
    static DATA: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());

    let mut p = DATA.load(Ordering::Acquire);
    let tid = std::thread::current().id().as_u64();

    if p.is_null() {
        let data = Box::new(Data::default());
        p = Box::into_raw(data);
        match DATA.compare_exchange(
            std::ptr::null_mut(),
            p,
            Ordering::Release, // success ordering (store)
            Ordering::Acquire, // failure ordering (load)
        ) {
            Ok(k) => {
                println!("Data initialized by thread@{tid}");
                assert!(k.is_null());
            }
            Err(e) => {
                // If we get here, another thread has already initialized the key.
                println!(
                    "Data is already initialised by other thread -- data : {:?}",
                    e
                );
                // Failed to store the pointer.
                // Another thread must have stored a pointer first.
                // We have to free the pointer we created.
                // Safety: p comes from Box::into_raw, and wasn't shared with other threads.
                drop(unsafe { Box::from_raw(p) });
                p = e;
            }
        }
    }

    // Safety: p is not null, and points to a properly initialized Data.
    unsafe { &*p }
}

fn main() {
    // One time initialization.
    for _ in 0..10 {
        std::thread::spawn(|| {
            let data = get_data();
            println!("Data: {:?}", data);
        })
        .join()
        .unwrap();
    }
}
