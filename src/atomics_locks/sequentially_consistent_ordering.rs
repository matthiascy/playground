use std::sync::atomic::{AtomicBool, Ordering};

static A: AtomicBool = AtomicBool::new(false);
static B: AtomicBool = AtomicBool::new(false);

static mut S: String = String::new();

fn main() {
    let ta = std::thread::spawn(|| {
        A.store(true, Ordering::SeqCst);
        if !B.load(Ordering::SeqCst) {
            unsafe {
                S.push_str("!");
            }
        }
    });

    let tb = std::thread::spawn(|| {
        B.store(true, Ordering::SeqCst);
        if !A.load(Ordering::SeqCst) {
            unsafe {
                S.push_str("?");
            }
        }
    });

    ta.join().unwrap();
    tb.join().unwrap();

    println!("{}", unsafe { &S });
}
