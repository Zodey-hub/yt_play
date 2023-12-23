use http_req::{request::Request, uri::Uri};

use std::convert::TryFrom;
use std::{error::Error, io};

use tabled::Table;
use yt_play::{get_videos, Video};

pub fn get_searched_video() -> Result<Video, Box<dyn Error>> {
    print!("Search for: ");
    io::Write::flush(&mut io::stdout())?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let input = input.trim().replace(' ', "+");

    let search_url = format!(
        "{}{}",
        "https://www.youtube.com/results?search_query=", input
    );

    let mut webpage_buffer = Vec::new();
    let uri = Uri::try_from(search_url.as_str())?;

    Request::new(&uri)
        .header("Accept-Language", "en-US")
        .send(&mut webpage_buffer)?;

    let videos = get_videos(&String::from_utf8(webpage_buffer)?)?;

    println!("{}", Table::new(&videos));

    loop {
        println!("Please select the music by it's index!");
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        match input.trim().parse::<usize>() {
            Ok(number) => {
                if number > 4 {
                    eprintln!("That is not a valid number. Please try again.");
                    continue;
                }
                return Ok(videos[number].clone());
            }
            Err(_) => {
                eprintln!("That is not a valid number. Please try again.");
                continue;
            }
        }
    }
}
