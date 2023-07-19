use std::sync::atomic::{AtomicU64, Ordering};

fn main() {
    // One time initialization.
    for _ in 0..10 {
        std::thread::spawn(|| {
            let key = get_key();
            println!("Key: {}", key);
        })
        .join()
        .unwrap();
    }
}

fn get_key() -> u64 {
    static KEY: AtomicU64 = AtomicU64::new(0);
    let key = KEY.load(Ordering::Relaxed);
    if key == 0 {
        println!("Initializing key...");
        let new_key = generate_key();
        match KEY.compare_exchange(0, new_key, Ordering::Relaxed, Ordering::Relaxed) {
            Ok(_) => {
                println!("Key initialized.");
                new_key
            }
            Err(k) => {
                // If we get here, another thread has already initialized the key.
                // We just return the key that the other thread has initialized.
                println!("Key is already initialised by other thread, key: {}", k);
                k
            }
        }
    } else {
        println!("Key already initialized.");
        key
    }
}

const fn generate_key() -> u64 {
    0xFF45
}
