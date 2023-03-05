#![no_std]
#![feature(core_intrinsics)]

mod engine;
mod game;
mod utils;

use core::{arch::wasm32, panic::PanicInfo};

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    game::event_loop::update();
}
