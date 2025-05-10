use std::env;
use std::process::Command;

use tabled::Tabled;

#[allow(non_snake_case)]
#[derive(Tabled, Clone)]
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
    pub fn new() -> Self {
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

    pub fn bestaudio_direct_link(&self) -> String {
        let output = Command::new("yt-dlp")
            .arg(format!("https://youtu.be/{}", self.VideoID))
            .arg("-f")
            .arg("bestaudio")
            .arg("-g")
            .output()
            .expect("Failed to call yt-dlp!");

        String::from_utf8(output.stdout).unwrap()
    }

    pub fn play_best_audio(&self) {
        let player =
            env::var("yt_play_player").expect("No \"yt_play_player\" env variable was set!");

        Command::new(&player)
            .arg(self.bestaudio_direct_link())
            .output()
            .expect(&format!("Failed to call {}!", player));
    }
}

impl Default for Video {
    fn default() -> Self {
        Video::new()
    }
}

pub fn get_videos(search_response_data: serde_json::Value) -> [Video; 5] {
    let mut videos: [Video; 5] = Default::default();
    let mut idx = 0;

    let items = search_response_data
        .pointer(
            "/contents/twoColumnSearchResultsRenderer/primaryContents/sectionListRenderer/contents",
        )
        .and_then(|c| c.get(0))
        .and_then(|c| c.get("itemSectionRenderer"))
        .and_then(|c| c.get("contents"))
        .and_then(|c| c.as_array())
        .expect("YouTube changed it's website structure, the program needs to be updated!");

    for item in items.iter() {
        if idx >= 5 {
            break;
        }

        let video_renderer = match item.get("videoRenderer") {
            Some(v) => v,
            None => continue,
        };

        let title = video_renderer["title"]["runs"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let uploader = video_renderer["ownerText"]["runs"][0]["text"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let length = video_renderer["lengthText"]["simpleText"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let view = video_renderer["viewCountText"]["simpleText"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        let uploaded = video_renderer["publishedTimeText"]["simpleText"]
            .as_str()
            .unwrap_or("YouTube") // ezek a videók YouTube által vannak generálva így nincsen feltöltési dátumuk
            .to_string();

        let video_id = video_renderer["videoId"]
            .as_str()
            .unwrap_or_default()
            .to_string();

        videos[idx] = Video {
            Index: idx,
            Title: title,
            Uploader: uploader,
            Length: length,
            View: view,
            Uploaded: uploaded,
            VideoID: video_id,
        };

        idx += 1;
    }

    videos
}
