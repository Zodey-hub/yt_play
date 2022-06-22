use std::io;

pub fn scanf(mut buffer: &mut String) {
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");
}

pub fn give_text_between(source: &str, before: &str, after: &str) -> String {
    let mut start_bytes = source.find(before).unwrap();
    start_bytes += before.len();
    let end_bytes = source[start_bytes..].find(after).unwrap();
    source[start_bytes..start_bytes + end_bytes]
        .trim()
        .to_string()
}

// pub fn get_occurrences(source: &String, string_to_find: &str) -> usize {
//     source.matches(string_to_find).count()
// }
