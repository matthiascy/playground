use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

static DATA_ATOMIC: AtomicU64 = AtomicU64::new(0);
static mut DATA_NON_ATOMIC: u64 = 0;

static READY: AtomicBool = AtomicBool::new(false);

fn main() {
    println!("Type 'atomic', 'non-atomic' or 'mutex'.");
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "atomic" => {
                atomic_data();
                break;
            }
            "non-atomic" => {
                non_atomic_data();
                break;
            }
            "mutex" => {
                mutex_lock();
                break;
            }
            _ => {
                println!("unknown command");
                break;
            }
        }
    }
}

fn atomic_data() {
    std::thread::spawn(|| {
        DATA_ATOMIC.store(42, Ordering::Relaxed);
        READY.store(true, Ordering::Release); // Everything from before this store..
    });

    while !READY.load(Ordering::Acquire) {
        // .. is visible after this loads `true`.
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("Waiting for data to be ready...");
    }

    println!("{}", DATA_ATOMIC.load(Ordering::Relaxed));
}

fn non_atomic_data() {
    std::thread::spawn(|| {
        // Safety: nothing else is accessing DATA_NON_ATOMIC.
        // because we haven't set the READY flag yet.
        unsafe {
            DATA_NON_ATOMIC = 1234;
        }
        READY.store(true, Ordering::Release);
    });

    while !READY.load(Ordering::Acquire) {
        std::thread::sleep(std::time::Duration::from_millis(100));
        println!("Waiting for data to be ready...");
    }

    // Safety: nothing else is accessing DATA_NON_ATOMIC.
    println!("{}", unsafe { DATA_NON_ATOMIC });
}

static mut DATA_MUTEX: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn mutex_lock() {
    std::thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(|| {
                if LOCKED
                    .compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
                    .is_ok()
                {
                    // Safety: nothing else is accessing DATA_MUTEX.
                    unsafe {
                        DATA_MUTEX.push_str("!");
                    }
                    LOCKED.store(false, Ordering::Release);
                }
            });
        }
    });
    // Safety: nothing else is accessing DATA_MUTEX.
    unsafe {
        println!("{}", DATA_MUTEX);
    }
}
