use std::{
    sync::atomic::{self, AtomicBool, Ordering},
    time::Duration,
};

const N: usize = 100;

static mut DATA: [u64; N] = [0; N];

const ATOMIC_FALSE: AtomicBool = AtomicBool::new(false);
static READY: [AtomicBool; N] = [ATOMIC_FALSE; N];

fn main() {
    for i in 0..N {
        std::thread::spawn(move || {
            let data = i as u64 + 2u64.pow(i as u32 % 16);
            unsafe {
                DATA[i] = data;
            }
            std::thread::sleep(Duration::from_millis((20 * i as u64) % 100));
            READY[i].store(true, Ordering::Release);
        });
    }

    std::thread::sleep(Duration::from_millis(50));
    let ready: [bool; N] = std::array::from_fn(|i| READY[i].load(Ordering::Relaxed));
    if ready.contains(&true) {
        atomic::fence(Ordering::Acquire);
        for i in 0..N {
            if ready[i] {
                println!("data[{i}]: {}", unsafe { DATA[i] });
            }
        }
    }
}
