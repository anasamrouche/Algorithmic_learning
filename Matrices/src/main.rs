#![allow(warnings)]

mod matrices;
mod miscellaneous;
mod naive;
mod strassen;

use matrices::Matrix;
use naive::multiply_naive;
use strassen::strassen;
use std::{time::Instant, vec};
use rand::random_range;

fn main() {
    const SIZE:usize = 32;
    
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

    assert!(D.core.len() == D.core[0].len() && E.core.len() == E.core[0].len(), "{}, {}; {}, {}", D.core.len(), D.core[0].len(), E.core.len(), E.core[0].len());

    let mut now = Instant::now();
    let F1 = strassen(&D, &E);
    let mut elapsed = now.elapsed();
    println!("{:?} elapsed for strassen algorithm.", elapsed);
    now = Instant::now();
    let F2 = multiply_naive(&D,&E);
    elapsed = now.elapsed();
    println!("{:?} elapsed for the naive algorithm.", elapsed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn are_equal() {
        const SIZE:usize = 32;
    
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