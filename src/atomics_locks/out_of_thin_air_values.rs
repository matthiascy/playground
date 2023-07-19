use std::sync::atomic::{AtomicI32, Ordering};

static X: AtomicI32 = AtomicI32::new(0);
static Y: AtomicI32 = AtomicI32::new(0);

fn main() {
    let ta = std::thread::spawn(|| {
        let x = X.load(Ordering::Relaxed);
        if x == 42 {
            Y.store(x, Ordering::Relaxed);
        }
    });

    let tb = std::thread::spawn(|| {
        let y = Y.load(Ordering::Relaxed);
        if y == 42 {
            X.store(y, Ordering::Relaxed);
        }
    });

    ta.join().unwrap();
    tb.join().unwrap();
    assert_eq!(X.load(Ordering::Relaxed), 0);
    assert_eq!(Y.load(Ordering::Relaxed), 0);
}
