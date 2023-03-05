use core::f32::consts::TAU;
pub use core::f32::consts::{FRAC_PI_2, PI};

const FIVE_PI_SQUARED: f32 = 5.0 * (PI * PI);

pub fn sinf(mut x: f32) -> f32 {
    let y = x / TAU;
    let z = y - floorf(y);
    x = z * TAU;

    let sinf_imp = |x: f32| -> f32 {
        // these magic numbers were discovered 1400 years ago!
        (16.0 * x * (PI - x)) / (FIVE_PI_SQUARED - (4.0 * x * (PI - x)))
    };

    if x > PI {
        -sinf_imp(x - PI)
    } else {
        sinf_imp(x)
    }
}

pub fn cosf(x: f32) -> f32 {
    sinf(x + FRAC_PI_2)
}

pub fn tanf(x: f32) -> f32 {
    sinf(x) / cosf(x)
}

pub fn sqrtf(x: f32) -> f32 {
    unsafe { core::intrinsics::sqrtf32(x) }
}

pub fn floorf(x: f32) -> f32 {
    unsafe { core::intrinsics::floorf32(x) }
}

pub fn ceilf(x: f32) -> f32 {
    unsafe { core::intrinsics::ceilf32(x) }
}

pub fn fabsf(x: f32) -> f32 {
    unsafe { core::intrinsics::fabsf32(x) }
}
