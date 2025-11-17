use std::{
    sync::{Arc, Mutex},
    thread,
    // time::Duration,
};

fn main() {
    // let message = String::from("Hello World!");
    // Move keyword takes any value used in the thread
    // that comes from outside and passes it into the
    // thread by value, not reference
    // let handle = thread::spawn(move || {
    //     for _ in 1..=5 {
    //         println!("{}", message);
    //         thread::sleep(Duration::from_secs(1));
    //     }
    // });

    // Creates a shared state variable that is atomically
    // reference counted and mutually exclusive within threads
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let in_handle = thread::spawn(move || {
            // Locked before mutating so that other threads do not modify values in it
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("{}", *num);
        });
        handles.push(in_handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // let counter = Mutex::new(0);

    // let handle = thread::spawn(move || {
    //     let mut num = counter.lock().unwrap();
    //     *num += 1;
    // });

    // ERROR: counter was moved into the thread above!
    // We can't use it again here
    // handle.join().unwrap();
    // handle.join().unwrap();
    println!("Result: {}!", *counter.lock().unwrap());
}
