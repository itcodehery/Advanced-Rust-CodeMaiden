// Project: Multi-threaded Download Manager Simulator
// The Problem
// Build a program that simulates downloading multiple files concurrently and tracks progress. Here's what it should do:
// Requirements
// 1. File Download Simulation

// You have 5 "files" to download (just strings like "file1.zip", "file2.mp4", etc.)
// Each file takes a random amount of time to download (simulate with thread::sleep - use random durations between 1-3 seconds)
// Each file has a random size (generate random bytes between 1000-5000)

// 2. Worker Threads

// Create 3 worker threads that download files concurrently
// Workers should pick files from a shared queue/list
// When a worker finishes a file, it should pick the next one until all are done

// 3. Progress Tracking (Arc + Mutex)

// Track these shared statistics:

// Total bytes downloaded so far
// Number of files completed
// Number of files failed (simulate 20% random failure rate)

// 4. Results Collection (Channels)

// When a worker finishes a download, send a result message containing:

// File name
// Success or failure
// Bytes downloaded
// Which worker thread completed it

// Have a separate "reporter" thread that receives these messages and prints them

// 5. Live Monitor (Another thread + Arc/Mutex)

// Create a monitoring thread that runs every 500ms
// It should read the shared stats and print current progress
// Format: "Progress: 3/5 files, 8500 bytes downloaded, 1 failed"

// 6. Final Summary

// After all downloads complete, print a final report:

// Total successful downloads
// Total failed downloads
// Total bytes downloaded
// Total time taken
use std::{thread, time::Duration};
use text_io::*;

struct File {
    name: String,
    size: u32,
    ttp: u32,
    processed: bool,
}

impl File {
    fn new(name: String, size: u32, ttp: u32) -> Self {
        Self {
            name,
            size,
            ttp,
            processed: false,
        }
    }
}

fn main() {
    let mut files: Vec<File> = Vec::new();
    for _ in 0..3 {
        println!("Enter the name of the file: ");
        let name: String = read!();
        println!("Enter the size of the file: ");
        let size: u32 = read!();
        println!("Enter the time to process of the file (in secs): ");
        let ttp: u32 = read!();
        let file = File::new(name, size, ttp);
        files.push(file);
    }

    // let r_state = ;
    let worker1 = thread::spawn(move || {
        for mut file in files {
            if !file.processed {
                println!("Processing {}", file.name);
                println!("Size of file: {}", file.size);

                thread::sleep(Duration::from_secs(file.size.into()));
                file.processed = true;
                println!();
            }
        }
    });

    let worker2 = thread::spawn(move || {
        for mut file in files {
            if !file.processed {
                println!("Thread #2 Processing {}", file.name);
                println!("Size of file: {}", file.size);

                thread::sleep(Duration::from_secs(file.size.into()));
                file.processed = true;
                println!();
            }
        }
    });

    let worker3 = thread::spawn(move || {
        for mut file in files {
            if !file.processed {
                println!("Thread #3 Processing {}", file.name);
                println!("Size of file: {}", file.size);

                thread::sleep(Duration::from_secs(file.size.into()));
                file.processed = true;
                println!();
            }
        }
    });
}
