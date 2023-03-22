mod input_and_output;
mod string_utils;
mod video_utils;

use std::process::Command;

fn main() {
    let video_id = input_and_output::get_searched_video_id();

    Command::new("vlc")
        .arg("-I rc")
        .arg(video_utils::get_video_direct_link(&video_id))
        .arg("--no-video")
        .arg("--play-and-exit")
        .spawn()
        .expect("Failed to start VLC! Maybe it's not in the path...");
}
