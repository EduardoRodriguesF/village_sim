pub mod prelude;
pub mod shapes;

pub fn approach(mut from: f32, to: f32, amount: f32) -> f32 {
    if from < to {
        from += amount;

        if from > to {
            return to;
        }
    } else {
        from -= amount;

        if from < to {
            return to;
        }
    }

    from
}
