#![no_std]
#![feature(core_intrinsics)]

mod engine;
mod game;
mod utils;

use core::{arch::wasm32, panic::PanicInfo};
use engine::input;
use game::state::STATE;

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    STATE.update(
        input::button_up_pressed(),
        input::button_down_pressed(),
        input::button_left_pressed(),
        input::button_right_pressed(),
        input::button_z_pressed(),
        input::button_x_pressed(),
        input::mouse_x(),
        input::mouse_left_pressed(),
    );

    engine::draw(&mut STATE);
}
