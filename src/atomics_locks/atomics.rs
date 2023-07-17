fn main() {
    // example_stop_flag();
    example_progress_reporting()
}

fn example_load_store_stop_flag() {
    use std::sync::atomic::{AtomicBool, Ordering};
    static STOP: AtomicBool = AtomicBool::new(false);

    println!("Type 'help' for a list of commands.");

    // Spawn a thread to do the work.
    let background_thread = std::thread::spawn(move || {
        while !STOP.load(Ordering::Relaxed) {
            // Do some work.
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
    });

    // Use the main thread to listen for user input.
    for line in std::io::stdin().lines() {
        match line.unwrap().as_str() {
            "help" => println!("commands: stop, help"),
            "stop" => break,
            cmd => println!("unknown command: {cmd}"),
        }
    }

    STOP.store(true, Ordering::Relaxed);

    background_thread.join().unwrap();
}

fn example_load_store_progress_reporting() {
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

fn example_load_store_lazy_initialisation() {
    use std::sync::atomic::{AtomicU64, Ordering};

    fn get_x() -> u64 {
        static X: AtomicU64 = AtomicU64::new(0);
        let mut x = X.load(Ordering::Relaxed);
        if x == 0 {
            x = {
                // Do some expensive computation to initialise x.
                std::thread::sleep(std::time::Duration::from_secs(1));
                42
            };
            X.store(x, Ordering::Relaxed);
        }
        x
    }
}

fn example_fetch_and_modify() {}
