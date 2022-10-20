mod stringutils;

use http_req::{request::Request, response::Headers, uri::Uri};
use std::{process::Command, io};
use tabled::{Table, Tabled};

#[derive(Tabled, Clone)]
struct Video {
    Index: usize,
    Title: String,
    Uploader: String,
    Length: String,
    View: String,
    Uploaded: String,
    VideoID: String,
}

impl Default for Video {
    fn default() -> Self {
        Self {
            Index: 0,
            Title: String::new(),
            Uploader: String::new(),
            Length: String::new(),
            View: String::new(),
            Uploaded: String::new(),
            VideoID: String::new(),
        }
    }
}

fn main() {
    print!("Search for: ");
    io::Write::flush(&mut io::stdout()).expect("flush failed!");
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input = input.trim().replace(" ", "+");

    let search_url = format!(
        "{}{}",
        "https://www.youtube.com/results?search_query=",
        input
    );

    let uri: Uri = Uri::try_from(search_url.as_ref()).unwrap();
    let mut headers = Headers::default_http(&uri);
    // headers.insert("Accept-Language", "en-US");

    let mut webpage_buffer = Vec::new();
    let _response = Request::new(&uri)
        .headers(headers)
        .send(&mut webpage_buffer)
        .unwrap();
    let mut webpage_source = String::from_utf8_lossy(&webpage_buffer).to_string();

    // let mut videos: Vec<Video> = vec![Video::default(); 5];
    let mut videos: [Video; 5] = Default::default();

    const BEFORE: &str = "\"title\":{\"runs\":[";
    const AFTER: &str = "\",\"params\":";
    let mut start: usize;
    let mut end: usize;
    let mut all_video_data: String;

    for i in 0..5 {
        start = webpage_source.find(BEFORE).unwrap();
        start += BEFORE.len();
        end = webpage_source[start..].find(AFTER).unwrap();
        all_video_data = webpage_source[start..start + end].to_string();
        
        videos[i].Index = i;
        videos[i].Title = stringutils::give_text_between(&all_video_data, "{\"label\":\"", " készítette:");
        videos[i].Uploader = stringutils::give_text_between(&all_video_data, "{\"runs\":[{\"text\":\"", "\"");
        videos[i].Length = stringutils::give_text_between(&all_video_data, "\"}},\"simpleText\":\"", "\"");
        videos[i].View = stringutils::give_text_between(&all_video_data,"viewCountText\":{\"simpleText\":\"", " ");
        if all_video_data.contains("\"publishedTimeText\":{\"simpleText\":\"") { // if the video is uploaded by yt, it doesn't have date
            videos[i].Uploaded = stringutils::give_text_between(&all_video_data, "\"publishedTimeText\":{\"simpleText\":\"", "\"",);
        } else {
            videos[i].Uploaded = "Unknown!".to_string();
        }
        videos[i].VideoID = stringutils::give_text_between(&all_video_data, ":{\"url\":\"/watch?v=", "\"");

        webpage_source = webpage_source[start + end..].to_string();
    }

    println!("{}", Table::new(videos.clone()).to_string());

    println!("Please select the music by it's ID.");

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line!");

    let input: usize = input
        .trim()
        .parse()
        .expect("Please give me correct string number!");

    let selected_video_link = format!(
        "{}{}",
        "https://www.youtube.com/watch?v=", videos[input].VideoID
    );

    Command::new("vlc")
        .arg("-I rc")
        .arg(selected_video_link)
        .arg("--no-video")
        .spawn()
        .expect("Failed to start VLC! Maybe it's not in the path...");
}
