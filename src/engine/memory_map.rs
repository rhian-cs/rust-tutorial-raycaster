// These values are stored at specific address, so we'll be using hardcoded pointers.
// See all values at: https://wasm4.org/docs/reference/memory/
pub const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
pub const GAMEPAD1: *const u8 = 0x16 as *const u8;
pub const MOUSE_X: *const i16 = 0x1a as *const i16;
pub const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;
