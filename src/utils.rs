use http_req::{request::Request, uri::Uri};

const SEARCH_URL: &str = "https://www.youtube.com/results?search_query=";

pub fn get_user_input(question: &str) -> String {
    print!("{}", question);
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input
}

pub fn search_youtube(query: &str) -> serde_json::Value {
    let query = query.trim().replace(' ', "+");

    let search_url = format!("{}{}", SEARCH_URL, query);

    let mut webpage_buffer = Vec::new();
    let uri = Uri::try_from(search_url.as_str()).unwrap();

    Request::new(&uri)
        .header("Accept-Language", "en-US")
        .send(&mut webpage_buffer)
        .expect("Failed to connect to YouTube servers!");

    let webpage_buffer = String::from_utf8(webpage_buffer).unwrap();

    let yt_data = give_text_between(&webpage_buffer, "ytInitialData = ", ";</script>")
        .expect("YouTube changed it's website structure, the program needs to be updated!");

    serde_json::from_str(&yt_data).unwrap()
}

pub fn give_text_between(source: &str, before: &str, after: &str) -> Result<String, &'static str> {
    let start_bytes = source.find(before).ok_or("Before string not found")? + before.len();
    let end_bytes = source[start_bytes..]
        .find(after)
        .ok_or("After string not found")?;

    Ok(source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string())
}
