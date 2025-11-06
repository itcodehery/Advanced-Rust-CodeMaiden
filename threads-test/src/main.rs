use std::{fmt::Display, thread, time::Duration};

#[allow(dead_code)]
pub struct Track {
    id: i32,
    name: String,
    duration: i32,
    effect: AudioEffect,
}

impl Track {
    pub fn new(id: i32, name: String, duration: i32, effect: AudioEffect) -> Self {
        Self {
            id,
            name,
            duration,
            effect,
        }
    }

    pub fn play(&self) {
        println!(
            "Now playing...\n{}\nWith Audio Effect: {}\n",
            self.name, self.effect
        );
        let mut minute = 0;
        let mut seconds = 0;
        for _ in 0..self.duration {
            seconds += 1;
            if seconds > 59 {
                seconds = 0;
                minute += 1;
            }
            println!(
                "{}:{}{}",
                minute,
                if seconds < 10 { "0" } else { "" },
                seconds
            );
            thread::sleep(Duration::from_millis(200));
        }
    }
}

pub enum AudioEffect {
    None,
    Reverb(i32),
    Delay(i32),
    Compression,
}

impl Display for AudioEffect {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            AudioEffect::None => f.write_str("None"),
            AudioEffect::Reverb(amount) => f.write_fmt(format_args!("{}% Reverb", amount)),
            AudioEffect::Delay(amount) => f.write_fmt(format_args!("{}% Delay", amount)),
            AudioEffect::Compression => f.write_str("Compression"),
        }
    }
}

fn process_tracks(tracks: &Vec<Track>) {
    for track in tracks {
        track.play();
    }
}

fn main() {
    let tracks: Vec<Track> = vec![
        Track::new(
            1,
            String::from("Never Gonna Give You Up"),
            70,
            AudioEffect::None,
        ),
        Track::new(
            2,
            String::from("Who Let The Dogs Out?"),
            64,
            AudioEffect::Reverb(12),
        ),
        Track::new(
            3,
            String::from("Gangsta's Paradise"),
            167,
            AudioEffect::Compression,
        ),
    ];

    process_tracks(&tracks);
}
