mod input_and_output;
mod string_utils;
mod video_utils;

use std::{fs, process::Command};

fn main() {
    let video_id = input_and_output::get_searched_video_id();

    video_utils::download_video(&video_id);

    Command::new("vlc")
        .arg("-I rc")
        .arg(format!("{}.mp3", video_id))
        .arg("--no-video")
		.arg("--play-and-exit")
        .output()
        .expect("Failed to start VLC! Maybe it's not in the path...");

    fs::remove_file(format!("{}.mp3", video_id)).unwrap();
}
