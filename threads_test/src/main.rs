use std::thread::{self, JoinHandle};
use std::time::Duration;

fn main() {
    // Explicitly handle thread block
    let handle: JoinHandle<()> = thread::spawn(|| {
        for i in 1..=10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }

    // The handle.join() function waits for the
    // associated thread to finish
    handle.join().unwrap();
}
