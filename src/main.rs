use fltk::{
    app,
    button::Button,
    frame::Frame,
    prelude::{GroupExt, WidgetBase, WidgetExt},
    window::Window, group::Flex,
};

mod video_utils;

#[derive(Debug, Clone, Copy)]
pub enum Message {
    FromPC,
    ToPC,
    Start,
}

fn main() {
    const PLAY: &str = "Play";
    let app = app::App::default();
    let screen_size = app::screen_size();

    let mut window = Window::new(
        0,
        0,
        screen_size.0 as i32,
        screen_size.1 as i32,
        "yt_play by Zodey",
    );
    let frame = Frame::default().with_size(screen_size.0 as i32, screen_size.1 as i32).center_of(&window);
    let mut flex = Flex::default().with_size(120, 140).center_of_parent().column();
    let mut song_1_button = Button::new(100, 50, 200, 50, PLAY);
    let mut song_2_button = Button::new(100, 150, 200, 50, PLAY);
    let mut song_3_button = Button::new(100, 250, 200, 50, PLAY);
    let mut song_4_button = Button::new(100, 350, 200, 50, PLAY);
    let mut song_5_button = Button::new(100, 150, 200, 50, PLAY);

    window.make_resizable(true);
    window.end();
    window.show();

    let (s, r) = app::channel::<Message>();

    song_1_button.emit(s, Message::FromPC);
    song_2_button.emit(s, Message::ToPC);
    song_3_button.emit(s, Message::Start);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::FromPC => {
                    println!("{:#?}", msg);
                }
                Message::ToPC => {
                    println!("{:#?}", msg);
                }
                Message::Start => {
                    /* println!(
                        "{:?}",
                        video_utils::get_videos(&video_utils::get_content(
                            "https://www.youtube.com/results?search_query=stellar+ashes"
                        ))[0]
                            .thumbnail
                    ); */

                    for suggestion in video_utils::suggest_yt_queries("olyan ő") {
                        println!("{}", suggestion);
                    }
                }
            }
        }
    }
}
