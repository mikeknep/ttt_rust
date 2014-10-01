mod ttt;

fn main() {
    let mut reader = std::io::stdin();
    ttt::console_ui::runner::start_playing_session(&mut reader);
}
