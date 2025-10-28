#[allow(dead_code)]
pub struct Track {
    id: i32,
    name: String,
    duration: i32,
    effect: AudioEffect,
}

impl Track {
    f
}

pub enum AudioEffect {
    None,
    Reverb(i32),
    Delay(i32),
    Compression,
}

fn process_track(track: &'static Track) {
    let thread = std::thread::spawn(|| {
        println!("Processing #{}: {}", track.id, track.name);
    });
    let _ = thread.join();
}

fn main() {
    let tracks: Vec<Track> = vec![];
}
