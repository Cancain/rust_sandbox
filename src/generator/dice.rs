use rand::prelude::*;

pub fn roll_custom(max: i32) -> i32 {
    let mut rng = thread_rng();
    let roll: i32 = rng.gen_range(1, max);
    roll
}
