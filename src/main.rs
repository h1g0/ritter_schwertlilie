extern crate rust_dxlib;
use rust_dxlib::*;
use std::ffi::CString;
/*
mod input_state;
use input_state::*;

mod player_ship;
use player_ship::*;

*/



fn main() {
    ///ウィンドウタイトル
    let WINDOW_TITLE = CString::new("ritter_schwertlilie ver. 0.1.0 alpha").unwrap();

    ///画面サイズ
    let WINDOW_SIZE: (i32,i32) = (800,600);
    let COLOR_BIT: i32 = 32;
    let REFRESH_RATE: i32 = 60;

    unsafe {
        //dx_SetMainWindowText(GetSjisStrPtr(WINDOW_TITLE));
        dx_SetUseCharCodeFormat(DX_CHARCODEFORMAT_UTF8);
        dx_SetMainWindowText(WINDOW_TITLE.as_ptr());
        dx_ChangeWindowMode(TRUE);
        dx_SetGraphMode(WINDOW_SIZE.0, WINDOW_SIZE.1, COLOR_BIT, REFRESH_RATE);
        dx_DxLib_Init();
        dx_SetDrawScreen(DX_SCREEN_BACK);

        let player = dx_LoadGraph("img/player.png");

        // メインループ( 裏画面を表画面に反映, メッセージ処理, 画面クリア )
        while dx_ScreenFlip() == 0 && dx_ProcessMessage() == 0 && dx_ClearDrawScreen() == 0 {
            dx_DrawGraph(400, 300, player, TRUE);
            dx_DrawString(0,0,CString::new("ほげほげ").unwrap().as_ptr(),dx_GetColor(255, 0, 0));
        }
        dx_DxLib_End();
    }
}