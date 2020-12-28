#[macro_use]
extern crate clap;

use clap::{App, Arg};
use claxon::FlacReader;
use cursive::views::{Dialog, TextView};
use rodio::{Decoder, OutputStream, Sink};
use std::ffi::OsStr;
use std::{fs::File, io::BufReader, path::Path};

fn get_file_extension(name: &str) -> Option<&str> {
    Path::new(name).extension().and_then(OsStr::to_str)
}

fn main() -> Result<(), std::io::Error> {
    let matches = App::new(crate_name!())
        .version(crate_version!())
        .version_short("v")
        .help_message("Show this help message.")
        .version_message("Show the application's version.")
        .arg(Arg::with_name("file").help("The input file to play.").required(true).index(1))
        .get_matches();

    let mut siv = cursive::default();

    let name = matches.value_of("file").unwrap();
    let file = File::open(name).unwrap();
    let file_ext = get_file_extension(name).unwrap();

    if file_ext != "flac" {
        println!("You did not provide a valid FLAC file.");
        return Ok(());
    }

    let (_s, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    let reader = FlacReader::open(name).unwrap();

    let title = reader.get_tag("TITLE").next().unwrap();
    let artist = reader.get_tag("ARTIST").next().unwrap();

    siv.add_layer(Dialog::around(TextView::new(format!("Playing {} by {}", title, artist))));

    sink.append(source);

    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
