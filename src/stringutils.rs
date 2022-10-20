pub fn give_text_between(source: &str, before: &str, after: &str) -> String {
    let mut start_bytes = source.find(before).unwrap();
    start_bytes += before.len();
    let end_bytes = source[start_bytes..].find(after).unwrap();
    source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string()
}
