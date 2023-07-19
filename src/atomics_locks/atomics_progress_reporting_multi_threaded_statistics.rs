use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::thread;

// Fetch-and-Modify

fn main() {
    let num_done = &AtomicUsize::new(0);
    let total_time = &AtomicU64::new(0);
    let max_time = &AtomicU64::new(0);

    thread::scope(|s| {
        s.spawn(move || {
            // Spawn four threads to do the work then report progress.
            for t in 0..4 {
                s.spawn(move || {
                    for i in 0..25 {
                        let start = std::time::Instant::now();
                        // Do some work.
                        thread::sleep(std::time::Duration::from_millis((t * 25 + i) as u64 * 10));
                        let time_taken = start.elapsed().as_millis() as u64;

                        num_done.fetch_add(1, Ordering::Relaxed);
                        total_time.fetch_add(time_taken, Ordering::Relaxed);
                        max_time.fetch_max(time_taken, Ordering::Relaxed);
                    }
                });
            }
        });

        // The main thread shows status updates, every half second.
        loop {
            let total_time = total_time.load(Ordering::Relaxed);
            let max_time = max_time.load(Ordering::Relaxed);
            let n = num_done.load(Ordering::Relaxed);
            if n == 0 {
                println!("Working... nothing done yet.")
            } else if n == 100 {
                break;
            } else {
                println!(
                    "Progress: {n}/100 done, {:?}ms average, {:?}ms peek",
                    total_time / n as u64,
                    max_time
                );
            }
            thread::sleep(std::time::Duration::from_secs_f32(0.5));
        }
    });

    print!("Done!");
}
