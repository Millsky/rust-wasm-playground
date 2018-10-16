use std::thread;
use std::time::Duration;

fn main() {
    println!("HELLO FROM COCURRENCY");
    // Spawn threads
    let handle = thread::spawn(|| {
        for i in 1..20 {
            // Each thread perform an op
            println!("Hi I spawned from the thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // Wait for all the threads to be done processing and end
    handle.join().unwrap();
}
