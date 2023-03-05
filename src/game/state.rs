mod get_view;
mod process_movement;

pub struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    previous_mouse_x: i16,
    previous_mouse_left_pressed: bool,
}

pub static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
    previous_mouse_x: 0,
    previous_mouse_left_pressed: false,
};

impl State {
    pub fn update(
        &mut self,
        keyboard_up_pressed: bool,
        keyboard_down_pressed: bool,
        keyboard_left_pressed: bool,
        keyboard_right_pressed: bool,
        keyboard_x_pressed: bool,
        keyboard_z_pressed: bool,
        mouse_x: i16,
        mouse_left_pressed: bool,
    ) {
        self.process_movement(
            keyboard_up_pressed,
            keyboard_down_pressed,
            keyboard_left_pressed,
            keyboard_right_pressed,
            keyboard_x_pressed,
            keyboard_z_pressed,
            mouse_x,
            mouse_left_pressed,
        );
    }
}
