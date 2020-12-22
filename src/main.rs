extern crate piston_window;
use piston_window::*;

mod input_state;
use input_state::*;

mod player_ship;
use player_ship::*;

///ウィンドウタイトル
const WINDOW_TITLE: &str = "ritter_schwertlilie";
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

    let mut input_state = InputState::new();
    let mut player = PlayerShip::new(
        Size {
            width: WINDOW_SIZE.width,
            height: WINDOW_SIZE.height,
        },
        Size {
            width: 32.0,
            height: 32.0,
        },
        Size {
            width: 2.0,
            height: 2.0,
        },
        320.0,
        240.0,
        3,
    );
    while let Some(e) = window.next() {
        match e {
            Event::Loop(Loop::Render(_)) => {
                window.draw_2d(&e, |c, g, _| {
                    clear([0.0, 0.0, 0.0, 1.0], g);
                    
                });
            }
            Event::Loop(Loop::Update(_)) => {
                player.move_by_input(&input_state);
            }
            Event::Input(i, _) => {
                if let Input::Button(key) = i {
                    input_state.set(&key);
                }
            }
            _ => {}
        }
    }
}
