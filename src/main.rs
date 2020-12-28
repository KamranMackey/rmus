#[macro_use]
extern crate clap;

mod constants;

use crate::constants::*;

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

    let filename = matches.value_of("file").unwrap();
    let file_ext = get_file_extension(filename).unwrap();

    if file_ext != "flac" {
        println!("You did not provide a valid FLAC file.");
        return Ok(());
    }

    let mut siv = cursive::default();

    let (_s, handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&handle).unwrap();
    let file = File::open(filename).unwrap();
    let source = Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);

    let reader = FlacReader::open(filename).unwrap();
    let title = reader.get_tag(FLAC_TITLE_TAG).next().unwrap();
    let artist = reader.get_tag(FLAC_ARTIST_TAG).next().unwrap();

    siv.add_layer(Dialog::around(TextView::new(format!("Playing {} by {}.", title, artist))));
    siv.add_global_callback('q', |s| s.quit());
    siv.run();

    Ok(())
}
