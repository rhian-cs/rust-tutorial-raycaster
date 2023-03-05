use super::State;

use crate::{
    game::map,
    utils::math::{cosf, sinf, PI},
};

const STEP_SIZE: f32 = 0.045;
const MOUSE_FACTOR: f32 = 0.01;

impl State {
    pub fn process_movement(
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
        let previous_position = (self.player_x, self.player_y);

        self.process_keyboard_movement(
            keyboard_up_pressed,
            keyboard_down_pressed,
            keyboard_left_pressed,
            keyboard_right_pressed,
            keyboard_x_pressed,
            keyboard_z_pressed,
        );
        self.process_mouse_movement(mouse_x, mouse_left_pressed);

        if self.is_colliding_with_wall() {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    fn process_keyboard_movement(
        &mut self,
        up_pressed: bool,
        down_pressed: bool,
        left_pressed: bool,
        right_pressed: bool,
        button_x_pressed: bool,
        button_z_pressed: bool,
    ) {
        if up_pressed {
            self.step_forward();
        }

        if down_pressed {
            self.step_back();
        }

        if right_pressed {
            self.step_right();
        }

        if left_pressed {
            self.step_left();
        }

        if button_z_pressed {
            self.turn_left();
        }

        if button_x_pressed {
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

    fn process_mouse_movement(&mut self, mouse_x: i16, mouse_left_pressed: bool) {
        if !self.previous_mouse_left_pressed && mouse_left_pressed {
            self.previous_mouse_x = mouse_x;
        }

        if mouse_left_pressed {
            self.turn_by_mouse_drag(mouse_x);
        }

        self.previous_mouse_left_pressed = mouse_left_pressed;
    }

    fn turn_by_mouse_drag(&mut self, mouse_x: i16) {
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
