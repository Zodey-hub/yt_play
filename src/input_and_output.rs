use std::io;

use http_req::request;
use tabled::Table;

use crate::video_utils;

pub fn get_searched_video_id() -> String {
    print!("Search for: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim().replace(' ', "+");

    let search_url = format!(
        "{}{}",
        "https://www.youtube.com/results?search_query=", input
    );

    let mut webpage_buffer = Vec::new();
    request::get(&search_url, &mut webpage_buffer).unwrap();

    let videos = video_utils::get_videos(&String::from_utf8_lossy(&webpage_buffer));

    println!("{}", Table::new(videos.clone()));

    loop {
        println!("Please select the music by it's index!");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.trim().parse::<usize>() {
            Ok(number) => {
                return String::from(&videos[number].VideoID);
            }
            Err(_) => {
                println!("That's not a valid number. Please try again.");
                continue;
            }
        }
    }
}
