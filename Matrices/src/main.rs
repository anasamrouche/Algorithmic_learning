#![allow(warnings)]

mod matrices;
mod miscellaneous;
mod naive;
mod strassen;

use matrices::Matrix;
use naive::multiply_naive;
use strassen::strassen;
use std::time::Instant;

fn main() {
    const size:usize = 32;
    
    let mut D:Matrix<i64> = Matrix{core:vec![vec![1; size]]};
    let mut E:Matrix<i64> = Matrix{core:vec![vec![1; size]]};

    for _ in 0..size-1 {
        let mut arr1 = [0i64; size];
        rand::fill(&mut arr1);
        let mut arr2 = [0i64; size];
        rand::fill(&mut arr2);

        let ve1 = arr1.to_vec();
        let ve2 = arr2.to_vec();
        D.core.push(ve1);
        E.core.push(ve2);
    }

    let mut now = Instant::now();
    let F1 = strassen(&D, &E);
    let mut elapsed = now.elapsed();
    println!("{:?} elapsed for strassen algorithm.", elapsed);
    now = Instant::now();
    let F2 = multiply_naive(&D,&E);
    elapsed = now.elapsed();
    println!("{:?} elapsed for the naive algorithm.", elapsed)
}
