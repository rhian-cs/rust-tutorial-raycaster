#![no_std]

use core::f32::consts::{FRAC_PI_2, PI};
use core::{arch::wasm32, panic::PanicInfo};
use libm::{ceilf, cosf, fabsf, floorf, sinf, sqrtf, tanf};

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

// These values are stored at specific address, so we'll be using hardcoded pointers.
const DRAW_COLORS: *mut u16 = 0x14 as *mut u16;
const GAMEPAD1: *const u8 = 0x16 as *const u8;
const MOUSE_X: *const u16 = 0x1a as *const u16;
const MOUSE_BUTTONS: *const u8 = 0x1e as *const u8;

const BUTTON_X: u8 = 0b00000001;
const BUTTON_Z: u8 = 0b00000010;
// Bit 2 is unused
// Bit 3 is unused
const BUTTON_LEFT: u8 = 0b00010000;
const BUTTON_RIGHT: u8 = 0b00100000;
const BUTTON_UP: u8 = 0b01000000;
const BUTTON_DOWN: u8 = 0b10000000;

const MOUSE_BUTTON_LEFT: u8 = 0b00000001;

const MAP: [u16; 8] = [
    0b1111111111111111,
    0b1000001010000101,
    0b1011100000110101,
    0b1000111010010001,
    0b1010001011110111,
    0b1011101001100001,
    0b1000100000001101,
    0b1111111111111111,
];

const FOV: f32 = PI / 2.7; // The player's field of view.
const HALF_FOV: f32 = FOV * 0.5; // Half the player's field of view.
const ANGLE_STEP: f32 = FOV / 160.0; // The angle between each ray.
const WALL_HEIGHT: f32 = 100.0; // A magic number.

const STEP_SIZE: f32 = 0.045;
const MOUSE_FACTOR: f32 = 0.01;

struct State {
    player_x: f32,
    player_y: f32,
    player_angle: f32,
    mouse_x: u16,
    mouse_left: bool,
}

static mut STATE: State = State {
    player_x: 1.5,
    player_y: 1.5,
    player_angle: 0.0,
    mouse_x: 0,
    mouse_left: false,
};

impl State {
    pub fn update(
        &mut self,
        up: bool,
        down: bool,
        left: bool,
        right: bool,
        mouse_left: bool,
        mouse_x: u16,
        button_x: bool,
        button_z: bool,
    ) {
        let previous_position = (self.player_x, self.player_y);

        if up {
            self.player_x += cosf(self.player_angle) * STEP_SIZE;
            self.player_y += -sinf(self.player_angle) * STEP_SIZE;
        }

        if down {
            self.player_x -= cosf(self.player_angle) * STEP_SIZE;
            self.player_y -= -sinf(self.player_angle) * STEP_SIZE;
        }

        if right {
            self.player_x += sinf(self.player_angle) * STEP_SIZE;
            self.player_y += cosf(self.player_angle) * STEP_SIZE;
        }

        if left {
            self.player_x -= sinf(self.player_angle) * STEP_SIZE;
            self.player_y -= cosf(self.player_angle) * STEP_SIZE;
        }

        if (!self.mouse_left && mouse_left) {
            self.mouse_x = mouse_x;
        }

        if mouse_left {
            let mouse_x_diff: i32 = mouse_x as i32 - self.mouse_x as i32;
            self.mouse_x = mouse_x;

            self.player_angle += (mouse_x_diff as f32 * MOUSE_FACTOR) % (PI * 2.0);
        }

        self.mouse_left = mouse_left;

        if button_z {
            self.player_angle -= STEP_SIZE;
        }

        if button_x {
            self.player_angle += STEP_SIZE;
        }

        // if moving us on this frame put us into a wall just revert it
        if point_in_wall(self.player_x, self.player_y) {
            (self.player_x, self.player_y) = previous_position;
        }
    }

    pub fn get_view(&self) -> [(i32, bool); 160] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.player_angle + HALF_FOV;

        let mut walls = [(0, false); 160];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * ANGLE_STEP;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist = self.horizontal_intersection(angle);
            let v_dist = self.vertical_intersection(angle);

            let (min_dist, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            // *wall = (WALL_HEIGHT / f32::min(h_dist, v_dist)) as i32;
            *wall = (
                (WALL_HEIGHT / (f32::min(h_dist, v_dist) * cosf(angle - self.player_angle))) as i32,
                shadow,
            )
        }

        walls
    }

    fn horizontal_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let up = fabsf(floorf(angle / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_y = if up {
            ceilf(self.player_y) - self.player_y
        } else {
            floorf(self.player_y) - self.player_y
        };

        let first_x = -first_y / tanf(angle);

        let dy = if up { 1.0 } else { -1.0 };
        let dx = -dy / tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually break and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.

            let current_x = next_x + self.player_x;
            let current_y = if up {
                next_y + self.player_y
            } else {
                next_y + self.player_y - 1.0
            };

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }

    /// Returns the nearest wall the ray intersects with on a vertical grid line.
    fn vertical_intersection(&self, angle: f32) -> f32 {
        // This tells you if the angle is "facing up"
        // regardless of how big the angle is.
        let right = fabsf(floorf((angle - FRAC_PI_2) / PI) % 2.0) != 0.0;

        // first_y and first_x are the first grid intersections
        // that the ray intersects with.
        let first_x = if right {
            ceilf(self.player_x) - self.player_x
        } else {
            floorf(self.player_x) - self.player_x
        };
        let first_y = -tanf(angle) * first_x;

        // dy and dx are the "ray extension" values mentioned earlier.
        let dx = if right { 1.0 } else { -1.0 };
        let dy = dx * -tanf(angle);

        // next_x and next_y are mutable values which will keep track
        // of how far away the ray is from the player.
        let mut next_x = first_x;
        let mut next_y = first_y;

        // This is the loop where the ray is extended until it hits
        // the wall. It's not an infinite loop as implied in the
        // explanation, instead it only goes from 0 to 256.
        //
        // This was chosen because if something goes wrong and the
        // ray never hits a wall (which should never happen) the
        // loop will eventually quit and the game will keep on running.
        for _ in 0..256 {
            // current_x and current_y are where the ray is currently
            // on the map, while next_x and next_y are relative
            // coordinates, current_x and current_y are absolute
            // points.
            let current_x = if right {
                next_x + self.player_x
            } else {
                next_x + self.player_x - 1.0
            };
            let current_y = next_y + self.player_y;

            // Tell the loop to quit if we've just hit a wall.
            if point_in_wall(current_x, current_y) {
                break;
            }

            // if we didn't hit a wall on this extension add
            // dx and dy to our current position and keep going.
            next_x += dx;
            next_y += dy;
        }

        // return the distance from next_x and next_y to the player.
        distance(next_x, next_y)
    }
}

#[no_mangle]
unsafe fn update() {
    STATE.update(
        *GAMEPAD1 & BUTTON_UP != 0,
        *GAMEPAD1 & BUTTON_DOWN != 0,
        *GAMEPAD1 & BUTTON_LEFT != 0,
        *GAMEPAD1 & BUTTON_RIGHT != 0,
        *MOUSE_BUTTONS & MOUSE_BUTTON_LEFT != 0,
        *MOUSE_X,
        *GAMEPAD1 & BUTTON_Z != 0,
        *GAMEPAD1 & BUTTON_X != 0,
    );

    for (x, wall) in STATE.get_view().iter().enumerate() {
        let (height, shadow) = wall;

        if *shadow {
            *DRAW_COLORS = 0x2;
        } else {
            *DRAW_COLORS = 0x3;
        }

        vline(x as i32, 80 - (height / 2), *height as u32);
    }
}

fn point_in_wall(x: f32, y: f32) -> bool {
    match MAP.get(y as usize) {
        Some(line) => (line & (0b1 << x as usize)) != 0,
        None => true, // Out of bounds address are always a wall
    }
}

fn distance(a: f32, b: f32) -> f32 {
    sqrtf((a * a) + (b * b))
}
