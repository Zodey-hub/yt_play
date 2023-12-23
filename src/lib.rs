use std::{error::Error, process::Command};

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

    pub fn bestaudio_direct_link(&self) -> Result<String, Box<dyn Error>> {
        let output = Command::new("yt-dlp")
            .arg(format!("https://youtu.be/{}", self.VideoID))
            .arg("-f")
            .arg("bestaudio")
            .arg("-g")
            .output()?;
        Ok(String::from_utf8(output.stdout)?)
    }

    pub fn play_best_audio(&self) -> Result<(), Box<dyn Error>> {
        Command::new("mpv")
            .arg(self.bestaudio_direct_link()?)
            .output()?;
        Ok(())
    }
}

impl Default for Video {
    fn default() -> Self {
        Video::new()
    }
}

pub fn get_videos(mut webpage_source: &str) -> Result<[Video; 5], Box<dyn Error>> {
    let mut videos: [Video; 5] = Default::default();

    const BEFORE: &str = "\"title\":{\"runs\":[";
    const AFTER: &str = "\",\"params\":";
    let mut start: usize;
    let mut end: usize;
    let mut all_video_data: String;

    for i in 0..5 {
        start = webpage_source
            .find(BEFORE)
            .ok_or("Failed to find title in the webpage!")?;
        start += BEFORE.len();
        end = webpage_source[start..]
            .find(AFTER)
            .ok_or("Failed to find the end of the title in the webpage!")?
            + 1;
        all_video_data = webpage_source[start..start + end].to_string();

        videos[i].Index = i;
        videos[i].Title = give_text_between(&all_video_data, "{\"label\":\"", " by")?;
        videos[i].Uploader = give_text_between(&all_video_data, "{\"runs\":[{\"text\":\"", "\"")?;
        videos[i].Length = give_text_between(&all_video_data, "\"}},\"simpleText\":\"", "\"")?;
        videos[i].View =
            give_text_between(&all_video_data, "viewCountText\":{\"simpleText\":\"", " ")?;

        if all_video_data.contains("\"publishedTimeText\":{\"simpleText\":\"") {
            // if the video is uploaded by yt, it doesn't have date
            videos[i].Uploaded = give_text_between(
                &all_video_data,
                "\"publishedTimeText\":{\"simpleText\":\"",
                "\"",
            )?;
        } else {
            videos[i].Uploaded = "YouTube".to_string();
        }
        videos[i].VideoID = give_text_between(&all_video_data, "videoId\":\"", "\"")?;

        webpage_source = &webpage_source[start + end..];
    }
    Ok(videos)
}

fn give_text_between(source: &str, before: &str, after: &str) -> Result<String, &'static str> {
    let start_bytes = source.find(before).ok_or("Before string not found")? + before.len();
    let end_bytes = source[start_bytes..]
        .find(after)
        .ok_or("After string not found")?;

    Ok(source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string())
}
