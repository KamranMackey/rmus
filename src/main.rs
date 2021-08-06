#[macro_use]
extern crate clap;

use clap::{App, Arg};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader};

struct Player {
    sink: Sink,
}

impl Player {
    pub fn new(handle: &OutputStreamHandle) -> Self {
        let sink = Sink::try_new(handle);
        match sink {
            Ok(sink) => Self { sink },
            Err(_) => {
                println!("Failed to initialize new sink.");
                Self { sink: Sink::new_idle().0 }
            }
        }
    }

    pub fn play_pause(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .version_short("v")
        .help_message("Show this help message.")
        .version_message("Show the application's version.")
        .arg(Arg::with_name("input").help("The input file to play. Also accepts directories.").required(true).index(1))
        .get_matches();

    let mut siv = cursive::default();

    let (_s, handle) = OutputStream::try_default().unwrap();
    let mut player = Player::new(&handle);
    let input = matches.value_of("input").unwrap();
    if std::fs::metadata(input)?.is_dir() {
        let path = std::fs::read_dir(input).unwrap();
        path.filter_map(Result::ok)
            .filter_map(|d| d.path().to_str().and_then(|f| if !f.ends_with(".jpg") { Some(d) } else { None }))
            .for_each(|f| {
                let filepath = f.path();
                let path = filepath.as_path().to_str().unwrap();
                let file = File::open(path).unwrap();
                let source = Decoder::new(BufReader::new(file)).unwrap();
                player.sink.append(source);
            });
    } else {
        let file = File::open(input).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        player.sink.append(source);
    }

    siv.add_global_callback('c', move |_| player.play_pause());
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
