fn main() {
    example_load_store_stop_flag();
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

fn example_load_store_lazy_initialisation() {
    // std::sync::Once and std::sync::OnceLock are more appropriate for this use case.
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
