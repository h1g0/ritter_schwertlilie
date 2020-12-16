extern crate piston_window;
use piston_window::*;

mod input_state;
mod player_ship;

///ウィンドウタイトル
const WINDOW_TITLE: &str = "PistonTest";
///画面サイズ
const WINDOW_SIZE: Size = Size {
    width: 640.0,
    height: 480.0,
};

fn main() {
    let mut window: PistonWindow = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
        .exit_on_esc(true)
        .vsync(true)
        .resizable(false)
        .samples(4)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));
    window.events.set_max_fps(60);
    window.events.set_ups(60);

    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                });
            }
            Event::Loop(Loop::Update(_)) => {}
            Event::Input(i, _) => {}
            _ => {}
        }
    }
}