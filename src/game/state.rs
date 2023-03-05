use super::event_loop::FrameState;

mod get_view;
mod process_movement;

pub struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    previous_mouse_x: i16,
    previous_mouse_left_pressed: bool,
    frame_state: Option<FrameState>,
}

pub static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
    previous_mouse_x: 0,
    previous_mouse_left_pressed: false,
    frame_state: None,
};

impl State {
    pub fn update(&mut self, frame_state: FrameState) {
        self.frame_state = Some(frame_state.clone());
        self.process_movement();
    }

    fn frame_state(&self) -> &FrameState {
        self.frame_state.as_ref().unwrap()
    }
}
