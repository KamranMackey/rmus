#[macro_use]
extern crate clap;

use clap::{App, Arg};
use ears::{Music, AudioController};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .version_short("v")
        .help_message("Show this help message.")
        .version_message("Show the application's version.")
        .arg(Arg::with_name("input").help("The input file to play. Also accepts directories.").required(true).index(1))
        .get_matches();

    let input = matches.value_of("input").unwrap();
    let mut siv = cursive::default();
    let mut music = Music::new(input).unwrap();
    music.play();

    siv.add_global_callback('c', move |_| play_pause(&mut music));
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}

fn play_pause(music: &mut Music) {
    if music.is_playing() {
        music.pause();
    } else {
        music.play();
    }
}
