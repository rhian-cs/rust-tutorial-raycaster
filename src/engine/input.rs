use super::memory_map;

const KEYBOARD_X_MASK: u8 = 0b00000001;
const KEYBOARD_Z_MASK: u8 = 0b00000010;
const KEYBOARD_LEFT_MASK: u8 = 0b00010000;
const KEYBOARD_RIGHT_MASK: u8 = 0b00100000;
const KEYBOARD_UP_MASK: u8 = 0b01000000;
const KEYBOARD_DOWN_MASK: u8 = 0b10000000;

const MOUSE_BUTTON_LEFT_MASK: u8 = 0b00000001;

pub unsafe fn keyboard_up_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_UP_MASK != 0
}

pub unsafe fn keyboard_down_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_DOWN_MASK != 0
}

pub unsafe fn keyboard_left_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_LEFT_MASK != 0
}

pub unsafe fn keyboard_right_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_RIGHT_MASK != 0
}

pub unsafe fn mouse_left_pressed() -> bool {
    *memory_map::MOUSE_BUTTONS & MOUSE_BUTTON_LEFT_MASK != 0
}

pub unsafe fn mouse_x() -> i16 {
    *memory_map::MOUSE_X
}

pub unsafe fn keyboard_z_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_Z_MASK != 0
}

pub unsafe fn keyboard_x_pressed() -> bool {
    *memory_map::GAMEPAD1 & KEYBOARD_X_MASK != 0
}
