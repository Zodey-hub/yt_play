mod input_and_output;

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let video = input_and_output::get_searched_video()?;
    video.play_best_audio()?;
    Ok(())
}
