use super::State;

use crate::{
    game::event_loop::FrameState,
    game::map,
    utils::math::{cosf, sinf, PI},
};

const STEP_SIZE: f32 = 0.045;
const MOUSE_FACTOR: f32 = 0.01;

impl State {
    pub fn process_movement(&mut self, frame_state: &FrameState) {
        let previous_position = (self.player_x, self.player_y);

        self.process_keyboard_movement(&frame_state);
        self.process_mouse_movement(&frame_state);

        if self.is_colliding_with_wall() {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    fn process_keyboard_movement(&mut self, frame_state: &FrameState) {
        if frame_state.keyboard_up_pressed {
            self.step_forward();
        }

        if frame_state.keyboard_down_pressed {
            self.step_back();
        }

        if frame_state.keyboard_right_pressed {
            self.step_right();
        }

        if frame_state.keyboard_left_pressed {
            self.step_left();
        }

        if frame_state.keyboard_z_pressed {
            self.turn_left();
        }

        if frame_state.keyboard_x_pressed {
            self.turn_right();
        }
    }

    fn step_forward(&mut self) {
        self.player_x += cosf(self.player_angle) * STEP_SIZE;
        self.player_y += -sinf(self.player_angle) * STEP_SIZE;
    }

    fn step_back(&mut self) {
        self.player_x -= cosf(self.player_angle) * STEP_SIZE;
        self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
    }

    fn step_right(&mut self) {
        self.player_x += sinf(self.player_angle) * STEP_SIZE;
        self.player_y += cosf(self.player_angle) * STEP_SIZE;
    }

    fn step_left(&mut self) {
        self.player_x -= sinf(self.player_angle) * STEP_SIZE;
        self.player_y -= cosf(self.player_angle) * STEP_SIZE;
    }

    fn process_mouse_movement(&mut self, frame_state: &FrameState) {
        let mouse_left_pressed = frame_state.mouse_left_pressed;
        let mouse_x = frame_state.mouse_x;

        if !self.previous_mouse_left_pressed && mouse_left_pressed {
            self.previous_mouse_x = mouse_x;
        }

        if frame_state.mouse_left_pressed {
            self.turn_by_mouse_drag(&frame_state);
        }

        self.previous_mouse_left_pressed = mouse_left_pressed;
    }

    fn turn_by_mouse_drag(&mut self, frame_state: &FrameState) {
        let mouse_x = frame_state.mouse_x;

        let mouse_drag: f32 = mouse_x as f32 - self.previous_mouse_x as f32;
        self.previous_mouse_x = mouse_x;

        self.player_angle += (mouse_drag * MOUSE_FACTOR) % (PI * 2.0);
    }

    fn turn_left(&mut self) {
        self.player_angle -= STEP_SIZE;
    }

    fn turn_right(&mut self) {
        self.player_angle += STEP_SIZE;
    }

    fn is_colliding_with_wall(&self) -> bool {
        map::point_in_wall(self.player_x, self.player_y)
    }
}
