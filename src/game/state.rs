use super::event_loop::FrameState;

mod get_view;
mod process_movement;

pub struct State<'a> {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    previous_mouse_x: i16,
    previous_mouse_left_pressed: bool,
    frame_state: Option<&'a FrameState>,
}

pub static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
    previous_mouse_x: 0,
    previous_mouse_left_pressed: false,
    frame_state: None,
};

impl<'a> State<'a> {
    pub fn update(&mut self, frame_state: &'a FrameState) {
        self.frame_state = Some(&frame_state);
        self.process_movement();
    }

    fn frame_keyboard_up_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_up_pressed
    }
    fn frame_keyboard_down_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_down_pressed
    }
    fn frame_keyboard_right_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_right_pressed
    }
    fn frame_keyboard_left_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_left_pressed
    }
    fn frame_keyboard_z_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_z_pressed
    }
    fn frame_keyboard_x_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).keyboard_x_pressed
    }
    fn frame_mouse_left_pressed(&self) -> bool {
        (*self.frame_state.unwrap()).mouse_left_pressed
    }
    fn frame_mouse_x(&self) -> i16 {
        (*self.frame_state.unwrap()).mouse_x
    }
}
