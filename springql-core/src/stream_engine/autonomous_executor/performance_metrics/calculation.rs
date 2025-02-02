// This file is part of https://github.com/SpringQL/SpringQL which is licensed under MIT OR Apache-2.0. See file LICENSE-MIT or LICENSE-APACHE for full license details.

use std::{
    cmp::max,
    ops::{Add, Mul, Sub},
};

pub(super) fn next_avg<T>(current_avg: T, current_n: u64, next_val: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T> + Mul<f32, Output = T> + Copy,
{
    current_avg + (next_val - current_avg) * (1.0 / ((current_n + 1) as f32))
}

pub(super) fn floor0<T>(v: T) -> u64
where
    T: Into<i64>,
{
    max(v.into(), 0i64) as u64
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::stream_engine::time::duration::{
        wall_clock_duration::WallClockDuration, SpringDuration,
    };

    use super::*;

    #[test]
    fn test_next_avg() {
        assert!(approx_eq!(f32, next_avg(1.0, 10000, 1.0), 1.0));
        assert!(approx_eq!(f32, next_avg(1.5, 2, 3.0), 2.0));
        assert!(approx_eq!(
            f32,
            next_avg(
                WallClockDuration::from_millis(1500),
                2,
                WallClockDuration::from_millis(3000)
            )
            .as_secs_f64() as f32,
            2.0
        ));
    }
}
