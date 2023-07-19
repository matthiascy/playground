fn main() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let main_thread = std::thread::current();

    let num_done = AtomicUsize::new(0);
    std::thread::scope(|s| {
        s.spawn(|| {
            for i in 0..100 {
                {
                    println!("Process item {i}");
                    std::thread::sleep(std::time::Duration::from_secs_f32(0.1));
                }
                num_done.store(i + 1, Ordering::Relaxed);
                main_thread.unpark(); // Wake up the main thread.
            }
        });

        // The main thread shows status updats.
        loop {
            let n = num_done.load(Ordering::Relaxed);
            if n == 100 {
                break;
            }
            println!("Progress: {n}/100 done");
            std::thread::park_timeout(std::time::Duration::from_secs(1));
        }
    });

    println!("Done!");
}
