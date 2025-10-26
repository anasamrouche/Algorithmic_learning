use rand::{distr::Distribution, rng, rngs::{SmallRng, ThreadRng}, Rng, SeedableRng};
use std::rc::{Rc};
use std::cell::RefCell;

use crate::matrices::Matrix;

pub fn is_power_of_2(n:usize) -> bool {
    let mut m:f32 = n as f32;
    while m != 1 as f32 {
        if (m as usize)%2 == 1 {
            return false
        }
        m /= 2 as f32;
        if m != m.round() {
            return false
        }
    }
    true
}

pub fn random_matrix(size: usize, low:i32, high:i32) -> Matrix<i32> {
    let rng = RefCell::new(SmallRng::from_os_rng());
    let matrix: Vec<Vec<i32>> = (0..size)
        .map(|_| {
            //let line:Vec<i32> = rng.borrow().clone().random_iter().take(size).collect();
            let line:Vec<i32> = (0..size)
                .map(|_|
                    rng.borrow().clone().gen_range((low..high)))
                .collect();
            line
        })
        .collect();
    Matrix{core:matrix}
}