pub fn get_content(url: &str) -> String {
    ureq::get(url).call().unwrap().into_string().unwrap()
}
