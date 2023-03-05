use super::State;

use crate::{
    game::map,
    utils::math::{cosf, sinf, PI},
};

const STEP_SIZE: f32 = 0.045;
const MOUSE_FACTOR: f32 = 0.01;

impl State<'_> {
    pub fn process_movement(&mut self) {
        let previous_position = (self.player_x, self.player_y);

        self.process_keyboard_movement();
        self.process_mouse_movement();

        if self.is_colliding_with_wall() {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    fn process_keyboard_movement(&mut self) {
        if self.frame_keyboard_up_pressed() {
            self.step_forward();
        }

        if self.frame_keyboard_down_pressed() {
            self.step_back();
        }

        if self.frame_keyboard_right_pressed() {
            self.step_right();
        }

        if self.frame_keyboard_left_pressed() {
            self.step_left();
        }

        if self.frame_keyboard_z_pressed() {
            self.turn_left();
        }

        if self.frame_keyboard_x_pressed() {
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

    fn process_mouse_movement(&mut self) {
        let mouse_left_pressed = self.frame_mouse_left_pressed();
        let mouse_x = self.frame_mouse_x();

        if !self.previous_mouse_left_pressed && mouse_left_pressed {
            self.previous_mouse_x = mouse_x;
        }

        if self.frame_mouse_left_pressed() {
            self.turn_by_mouse_drag();
        }

        self.previous_mouse_left_pressed = mouse_left_pressed;
    }

    fn turn_by_mouse_drag(&mut self) {
        let mouse_x = self.frame_mouse_x();

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
