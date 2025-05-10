mod utils;
mod video;

fn main() {
    let input = utils::get_user_input("Search for: ");

    let videos = video::get_videos(utils::search_youtube(&input));

    println!("{}", tabled::Table::new(&videos));

    loop {
        let input = utils::get_user_input("Please select the music by it's index!\n");
        match input.trim().parse::<usize>() {
            Ok(number) => {
                if number > 4 {
                    eprintln!("That is not a valid number. Please try again.");
                    continue;
                }
                videos[number].play_best_audio();
                break;
            }
            Err(_) => {
                eprintln!("That is not a valid number. Please try again.");
                continue;
            }
        }
    }
}
