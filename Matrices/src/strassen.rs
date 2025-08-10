use crate::matrices::Matrix;
use crate::miscellaneous::is_power_of_2;
use std::ops::{AddAssign, Mul, Sub};
use num_traits::Zero;

pub fn strassen<T: Zero + Copy + AddAssign + Sub<Output=T> + Mul<Output = T>>(A: &Matrix<T>, B:&Matrix<T>) -> Matrix<T> {
        let a: usize = A.core.len();
        let b: usize = B.core.len();
        let a_rows: usize = A.core[0].len();
        let b_rows: usize = B.core[0].len();
        //assert!(a == a_rows && b == a_rows && b == b_rows && is_power_of_2(a), "a : {}, a_rows : {}, b : {}, b_rows : {}", a, a_rows, b, b_rows);

        let mut C:Vec<Vec<T>> = vec![vec![T::zero(); a]; a];
        if a == 1 {
            C[0][0] = A.core[0][0] * B.core[0][0];
            return Matrix{core:C};
        }
    
        let A11 = A.get_block(0..a/2, 0..a/2);
        let A12 = A.get_block(0..a/2, a/2..a);
        let A21 = A.get_block(a/2..a, 0..a/2);
        let A22 = A.get_block(a/2..a, a/2..a);

        let B11 = B.get_block(0..a/2, 0..a/2);
        let B12 = B.get_block(0..a/2, a/2..a);
        let B21 = B.get_block(a/2..a, 0..a/2);
        let B22 = B.get_block(a/2..a, a/2..a);
        
        let S1 = B12.minus(&B22);
        let S2 = A11.plus(&A12);
        let S3 = A21.plus(&A22);
        let S4 = B21.minus(&B11);
        let S5 = A11.plus(&A22);
        let S6 = B11.plus(&B22);
        let S7 = A12.minus(&A22);
        let S8 = B21.plus(&B22);
        let S9 = A11.minus(&A21);
        let S10 = B11.plus(&B12);

        let P1 = strassen(&A11, &S1);
        let P2 = strassen(&S2, &B22);
        let P3 = strassen(&S3, &B11);
        let P4 = strassen(&A22, &S4);
        let P5 = strassen(&S5, &S6);
        let P6 = strassen(&S7, &S8);
        let P7 = strassen(&S9, &S10);

        let mut return_matrix = Matrix{core:C};

        return_matrix.insert_from(P5.plus(&P4).minus(&P2).plus(&P6), 0, 0);
        return_matrix.insert_from(P1.plus(&P2), 0, a/2);
        return_matrix.insert_from(P3.plus(&P4), a/2, 0);
        return_matrix.insert_from(P5.plus(&P1).minus(&P3).minus(&P7), a/2, a/2);

        return_matrix
    }