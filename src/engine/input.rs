use super::memory_map;

const BUTTON_X: u8 = 0b00000001;
const BUTTON_Z: u8 = 0b00000010;
const BUTTON_LEFT: u8 = 0b00010000;
const BUTTON_RIGHT: u8 = 0b00100000;
const BUTTON_UP: u8 = 0b01000000;
const BUTTON_DOWN: u8 = 0b10000000;

const MOUSE_BUTTON_LEFT: u8 = 0b00000001;

pub unsafe fn button_up_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_UP != 0
}

pub unsafe fn button_down_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_DOWN != 0
}

pub unsafe fn button_left_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_LEFT != 0
}

pub unsafe fn button_right_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_RIGHT != 0
}

pub unsafe fn mouse_left_pressed() -> bool {
    *memory_map::MOUSE_BUTTONS & MOUSE_BUTTON_LEFT != 0
}

pub unsafe fn mouse_x() -> i16 {
    *memory_map::MOUSE_X
}

pub unsafe fn button_z_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_Z != 0
}

pub unsafe fn button_x_pressed() -> bool {
    *memory_map::GAMEPAD1 & BUTTON_X != 0
}
