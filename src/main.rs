extern crate encoding_rs;
use encoding_rs::*;
extern crate rust_dxlib;
use rust_dxlib::*;

/*
mod input_state;
use input_state::*;

mod player_ship;
use player_ship::*;

*/

///ウィンドウタイトル
const WINDOW_TITLE: &str = "ritter_schwertlilie ver. 0.1.0 alpha";
///画面サイズ
const WINDOW_SIZE: (i32,i32) = (640,480);
const COLOR_BIT: i32 = 32;
const REFRESH_RATE: i32 = 60;

fn main() {

    unsafe {

        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(WINDOW_SIZE.0, WINDOW_SIZE.1, COLOR_BIT, REFRESH_RATE);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);
        /*
        let (sjis_str,_,_) = SHIFT_JIS.encode(WINDOW_TITLE);
        let title = sjis_str.as_ptr();
        dx_SetMainWindowText(title);
        */
        dx_SetMainWindowText(WINDOW_TITLE.as_ptr());

        // メインループ( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while dx_ScreenFlip() == 0 && dx_ProcessMessage() == 0 && dx_ClearDrawScreen() == 0 {
            
        }
        dx_DxLib_End();
    }
}
