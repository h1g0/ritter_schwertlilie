extern crate rust_dxlib;
use rust_dxlib::*;
use std::ffi::CString;

mod math;

mod input_state;
use input_state::*;

mod background;
use background::*;

mod player_ship;
use player_ship::*;

mod fps_manager;
use fps_manager::*;

fn main() {
    let window_title = CString::new("ritter_schwertlilie ver. 0.1.0 alpha").unwrap();

    let window_size: (i32, i32) = (800, 600);
    let color_bit: i32 = 32;
    let refresh_rate: i32 = 60;

    unsafe {
        dx_SetUseCharCodeFormat(DX_CHARCODEFORMAT_UTF8);
        dx_SetMainWindowText(window_title.as_ptr());
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(window_size.0, window_size.1, color_bit, refresh_rate);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);

        let mut player = PlayerShip::new(
            window_size,
            dx_LoadGraph("img/player.bmp"),
            (32, 32),
            (2, 2),
            400.0,
            300.0,
            3,
        );
        let mut fps = FpsManager::new(true, 60, dx_GetNowCount());
        let mut input = InputState::new();

        // メインループ( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while dx_ScreenFlip() == 0
            && dx_ProcessMessage() == 0
            && dx_ClearDrawScreen() == 0
            && input.esc == false
        {
            input.set(&InputState::get_key_state());

            player.move_by_input(&input);
            player.draw();

            fps.measure(dx_GetNowCount());
            let fps_color_val: i32 = if fps.get_percent() > 1.0 {
                255
            } else {
                (255.0 * fps.get_percent()) as i32
            };
            let fps_color = dx_GetColor(255, fps_color_val, fps_color_val);
            dx_DrawString(
                window_size.0 - 32,
                window_size.1 - 32,
                CString::new(format!("{}", fps.get())).unwrap().as_ptr(),
                fps_color,
            );
        }
        dx_DxLib_End();
    }
}
