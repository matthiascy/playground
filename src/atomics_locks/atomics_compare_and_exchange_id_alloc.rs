use std::sync::atomic::{AtomicU32, AtomicU8, Ordering};

fn increment(a: &AtomicU32) {
    let mut old = a.load(Ordering::Relaxed);
    loop {
        let new = old + 1;
        match a.compare_exchange(old, new, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => return,
            Err(x) => old = x,
        }
    }
}

fn allocate_new_id() -> u8 {
    static NEXT_ID: AtomicU8 = AtomicU8::new(0);
    let mut current = NEXT_ID.load(Ordering::Relaxed);
    loop {
        assert!(current < 100, "ID overflowed");
        match NEXT_ID.compare_exchange_weak(
            current,
            current + 1,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(val) => return val,
            Err(x) => current = x,
        }
    } // Returns the old value, which is the new ID after
}

fn main() {
    for _ in 0..150 {
        std::thread::spawn(move || {
            let tid = std::thread::current().id();
            let id = allocate_new_id();
            println!("Thread@{tid:?} got ID {id}");
        })
        .join()
        .unwrap();
    }
}
