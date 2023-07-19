use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

// Fetch-and-Modify

fn main() {
    let num_done = &AtomicUsize::new(0);

    thread::scope(|s| {
        // Spawn four threads to do the work then report progress.
        for t in 0..4 {
            s.spawn(move || {
                for i in 0..25 {
           n         // Do some work.
                    thread::sleep(std::time::Duration::from_secs_f32(
                        (t * 25 + i) as f32 * 0.01,
                    ));
                    num_done.fetch_add(1, Ordering::Relaxed);
                }
            });
        }

        // The main thread shows status updates, every half second.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Progress: {n}/100 done");
            thread::sleep(std::time::Duration::from_secs_f32(0.5));
        }
    });
}
