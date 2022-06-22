use std::process::Command;
mod httputils;
mod stringutils;

fn main() {
    println!("Search for: ");
    let mut search_for = String::new();
    stringutils::scanf(&mut search_for);
    search_for = search_for.replace(" ", "+");

    let mut search_url = String::from("https://www.youtube.com/results?search_query=");
    search_url.push_str(&search_for);

    print!("{}", search_url);

    let chunk = httputils::get_content(&search_url);

    let videio_id = stringutils::give_text_between(&chunk, "videoId\":\"", "\"");

    let mut video_url = String::from("https://www.youtube.com/watch?v=");
    video_url.push_str(&videio_id);

    let video_title = stringutils::give_text_between(
        &httputils::get_content(&video_url),
        "<meta itemprop=\"name\" content=\"",
        "\">",
    );

    println!("Playing: {} (URL: {})", video_title.trim(), video_url);

    Command::new("vlc")
        .arg("-I")
        .arg("rc")
        .arg(video_url)
        .arg("--no-video")
        .status()
        .expect("VLC failed to start, maybe it's not in the path.");
}
