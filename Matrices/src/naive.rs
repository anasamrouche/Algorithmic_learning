use crate::matrices::Matrix;
use std::ops::{AddAssign, Mul};
use num_traits::Zero;

pub fn multiply_naive<T:Zero + Clone + AddAssign + Mul<Output = T>> (A:&Matrix<T>, B:&Matrix<T>) -> Matrix<T> {
        let a: usize = A.core.len();
        let b: usize = B.core[0].len();
        let a_rows: usize = A.core[0].len();
        let b_columns: usize = B.core.len();
        assert!(a_rows == b_columns); 
        let mut C:Vec<Vec<T>> = vec![vec![(&T::zero()).clone(); b]; a];
        for i in 0..a {
            for j in 0..b {
                for k in 0..a_rows {
                    C[i][j] += A.core[i][k].clone() * B.core[k][j].clone();
                }
            }
        }
        let return_mat:Matrix<T> = Matrix{core:C};
        return_mat
    }