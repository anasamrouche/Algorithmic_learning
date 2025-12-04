#![allow(warnings)]

mod matrices;
mod naive;
mod strassen;
mod miscellaneous;

use matrices::Matrix;
use naive::multiply_naive;
use strassen::strassen;
use std::{time::Instant, vec};
use rand::{random_range, random_iter, rngs::SmallRng, distr::Distribution};
use num_traits::pow;
use std::rc::Rc;

fn benchmark() {
    const SIZES: [usize; 7] = [16, 32, 64, 128, 256, 512, 1024];
    let mut times_naive: [u128; 7] = [0; 7];
    let mut times_strassen: [u128; 7] = [0; 7];

    for (i, size) in SIZES.iter().enumerate() {
        let D = miscellaneous::random_matrix(*size, -128, 127);
        let E = miscellaneous::random_matrix(*size, -128, 127);

        let now = Instant::now();
        multiply_naive(&D, &E);
        let elapsed = now.elapsed();
        times_naive[i] = elapsed.as_millis();

        strassen(&D, &E);
        let elapsed = now.elapsed() - elapsed;
        times_strassen[i] = elapsed.as_millis();
    }
    println!("Résultats naïfs : {:#?}, résultats Strassen : {:#?}", times_naive, times_strassen);
}
fn main() {
    benchmark();
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn are_equal() {
        const SIZE:usize = 128;
    
        let mut rowA = vec![0; SIZE];
        let mut rowB = vec![0; SIZE];

        for i in 0..SIZE {
            rowA[i] = random_range(-1e3..1e3) as i32;
            rowB[i] = random_range(-1e3..1e3) as i32;
        }

        let mut D:Matrix<i32> = Matrix{core:vec![rowA.clone()]};
        let mut E:Matrix<i32> = Matrix{core:vec![rowB.clone()]};

        for _ in 0..SIZE-1 {
            for i in 0..SIZE {
                rowA[i] = random_range(-1e3..1e3) as i32;
                rowB[i] = random_range(-1e3..1e3) as i32;
            }

            D.core.push(rowA.clone());
            E.core.push(rowB.clone());
        }
        assert!(strassen(&D, &E) == multiply_naive(&D, &E));
    }

}