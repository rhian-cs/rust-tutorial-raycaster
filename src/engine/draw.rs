use super::colors::Color;
use super::external_functions::vline;
use crate::game::state::State;

use super::memory_map;

pub unsafe fn draw(state: &State) {
    for (x, wall) in state.get_view().iter().enumerate() {
        draw_column(x as i32, wall);
    }
}

unsafe fn draw_column(x: i32, &(height, shadow): &(i32, bool)) {
    if shadow {
        draw_color(Color::MediumLight);
    } else {
        draw_color(Color::MediumDark);
    }

    vline(x, 80 - (height / 2), height as u32);
}

unsafe fn draw_color(color: Color) {
    *memory_map::DRAW_COLORS = color as u16;
}
