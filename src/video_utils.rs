use crate::string_utils;
use http_req::request;
use tabled::Tabled;

#[derive(Tabled, Clone)]
#[allow(non_snake_case)]
pub struct Video {
    pub Index: usize,
    pub Title: String,
    pub Uploader: String,
    pub Length: String,
    pub View: String,
    pub Uploaded: String,
    pub VideoID: String,
}

impl Video {
    fn new() -> Self {
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
        end = webpage_source[start..].find(AFTER).unwrap();
        all_video_data = webpage_source[start..start + end].to_string();

        videos[i].Index = i;
        videos[i].Title =
            string_utils::give_text_between(&all_video_data, "{\"label\":\"", " készítette:");
        videos[i].Uploader =
            string_utils::give_text_between(&all_video_data, "{\"runs\":[{\"text\":\"", "\"");
        videos[i].Length =
            string_utils::give_text_between(&all_video_data, "\"}},\"simpleText\":\"", "\"");
        videos[i].View = string_utils::give_text_between(
            &all_video_data,
            "viewCountText\":{\"simpleText\":\"",
            " ",
        );
        if all_video_data.contains("\"publishedTimeText\":{\"simpleText\":\"") {
            // if the video is uploaded by yt, it doesn't have date
            videos[i].Uploaded = string_utils::give_text_between(
                &all_video_data,
                "\"publishedTimeText\":{\"simpleText\":\"",
                "\"",
            );
        } else {
            videos[i].Uploaded = "Unknown!".to_string();
        }
        videos[i].VideoID =
            string_utils::give_text_between(&all_video_data, ":{\"url\":\"/watch?v=", "\"");

        webpage_source = &webpage_source[start + end..];
    }
    videos
}

pub fn get_video_direct_link(id: &str) -> String {
    let mut api_response_buffer = Vec::new();
    request::get(
        format!(
            "https://youtube-dl-web.vercel.app/api/info?q={}&f=bestaudio",
            id
        ),
        &mut api_response_buffer,
    )
    .unwrap();
    let parsed = json::parse(&String::from_utf8_lossy(&api_response_buffer))
        .expect("Failed to parse JSON response!");
    parsed["url"].as_str().unwrap().to_string()
}
