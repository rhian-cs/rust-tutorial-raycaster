use super::state::STATE;
use crate::engine;
use crate::engine::input;

#[derive(Clone)]
pub struct FrameState {
    pub keyboard_up_pressed: bool,
    pub keyboard_down_pressed: bool,
    pub keyboard_left_pressed: bool,
    pub keyboard_right_pressed: bool,
    pub keyboard_z_pressed: bool,
    pub keyboard_x_pressed: bool,
    pub mouse_x: i16,
    pub mouse_left_pressed: bool,
}

pub unsafe fn update() {
    let frame_state = FrameState {
        keyboard_up_pressed: input::keyboard_up_pressed(),
        keyboard_down_pressed: input::keyboard_down_pressed(),
        keyboard_left_pressed: input::keyboard_left_pressed(),
        keyboard_right_pressed: input::keyboard_right_pressed(),
        keyboard_z_pressed: input::keyboard_z_pressed(),
        keyboard_x_pressed: input::keyboard_x_pressed(),
        mouse_x: input::mouse_x(),
        mouse_left_pressed: input::mouse_left_pressed(),
    };

    STATE.update(frame_state);

    engine::draw(&mut STATE);
}
