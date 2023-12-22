use std::process::Command;

use http_req::{request::Request, uri::Uri};
use regex::Regex;

pub struct Video {
    pub index: usize,
    pub title: String,
    pub uploader: String,
    pub length: String,
    pub view: String,
    pub uploaded: String,
    pub video_id: String,
    pub thumbnail: Vec<u8>,
}

impl Video {
    fn new() -> Self {
        Self {
            index: 0,
            title: String::new(),
            uploader: String::new(),
            length: String::new(),
            view: String::new(),
            uploaded: String::new(),
            video_id: String::new(),
            thumbnail: Vec::new(),
        }
    }

    fn bestaudio_direct_link(&self) -> String {
        let output = Command::new("yt-dlp")
            .arg(format!("https://youtu.be/{}", self.video_id))
            .arg("-f")
            .arg("bestaudio")
            .arg("-g")
            .output()
            .expect("Failed to start yt-dlp! Maybe it's not in the path...");
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn play_best_audio(&self) {
        Command::new("mpv").arg(self.bestaudio_direct_link());
    }
}

impl Default for Video {
    fn default() -> Self {
        Video::new()
    }
}

pub fn get_videos(mut webpage_source: &str) -> [Video; 5] {
    let mut videos: [Video; 5] = Default::default();

    const BEFORE: &str = "\"title\":{\"runs\":[";
    const AFTER: &str = "\",\"params\":";
    let mut start: usize;
    let mut end: usize;
    let mut all_video_data: String;

    for i in 0..5 {
        start = webpage_source.find(BEFORE).unwrap();
        start += BEFORE.len();
        end = webpage_source[start..].find(AFTER).unwrap() + 1;
        all_video_data = webpage_source[start..start + end].to_string();

        videos[i].index = i;
        videos[i].title = give_text_between(&all_video_data, "{\"label\":\"", " by");
        videos[i].uploader = give_text_between(&all_video_data, "{\"runs\":[{\"text\":\"", "\"");
        videos[i].length = give_text_between(&all_video_data, "\"}},\"simpleText\":\"", "\"");
        videos[i].view =
            give_text_between(&all_video_data, "viewCountText\":{\"simpleText\":\"", " ");

        if all_video_data.contains("\"publishedTimeText\":{\"simpleText\":\"") {
            // if the video is uploaded by yt, it doesn't have date
            videos[i].uploaded = give_text_between(
                &all_video_data,
                "\"publishedTimeText\":{\"simpleText\":\"",
                "\"",
            );
        } else {
            videos[i].uploaded = "YouTube".to_string();
        }
        videos[i].video_id = give_text_between(&all_video_data, "videoId\":\"", "\"");

        http_req::request::get(
            format!(
                "https://i.ytimg.com/vi/{}/maxresdefault.jpg",
                videos[i].video_id
            ),
            &mut videos[i].thumbnail,
        )
        .unwrap();

        webpage_source = &webpage_source[start + end..];
    }
    videos
}

fn give_text_between(source: &str, before: &str, after: &str) -> String {
    let mut start_bytes = source.find(before).unwrap();
    start_bytes += before.len();
    let end_bytes = source[start_bytes..].find(after).unwrap();
    source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string()
}

pub fn get_content(url: &str) -> String {
    let mut webpage_buffer = Vec::new();
    let uri = Uri::try_from(url).unwrap();

    let _response = Request::new(&uri)
        .header("Accept-Language", "en-US")
        .send(&mut webpage_buffer)
        .unwrap();
    String::from_utf8_lossy(&webpage_buffer).to_string()
}

pub fn suggest_yt_queries(input: &str) -> Vec<String> {
    let mut webpage_buffer = Vec::new();
    {
        // get unparsed queries
        let url = format!(
            "https://suggestqueries-clients6.youtube.com/complete/search?client=youtube&ds=yt&q={}",
            input.replace(" ", "+")
        );
        let uri = Uri::try_from(url.as_str()).unwrap();
        let _response = Request::new(&uri)
            .header(
                "User-Agent",
                "Mozilla/5.0 (X11; Linux x86_64; rv:109.0) Gecko/20100101 Firefox/119.0",
            )
            .send(&mut webpage_buffer)
            .unwrap();
    }

    let re = Regex::new(r#""(.*?)""#).unwrap();
    let webpage_text = String::from_utf8_lossy(&webpage_buffer);

    let mut queries: Vec<String> = re
        .captures_iter(&webpage_text)
        .skip(1) // Skip the first element
        .take(webpage_text.matches('"').count() - 3) // Take all but the last three elements
        .map(|cap| String::from(&cap[1]))
        .collect();

    queries.truncate(queries.len() - 3);
    queries
}
