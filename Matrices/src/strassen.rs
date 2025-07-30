use crate::matrices::Matrix;
use crate::miscellaneous::is_power_of_2;
use std::ops::{AddAssign, Mul, Sub};
use num_traits::Zero;

pub fn strassen<T: Zero + Clone + AddAssign + Sub<Output=T> + Mul<Output = T>>(A: &Matrix<T>, B:&Matrix<T>) -> Matrix<T> {
        let a: usize = A.core.len();
        let b: usize = B.core[0].len();
        let a_rows: usize = A.core[0].len();
        let b_columns: usize = B.core.len();
        assert!(a == b && b == a_rows && a_rows == b_columns && is_power_of_2(a));

        let mut C:Vec<Vec<T>> = vec![vec![(&T::zero()).clone(); a]; a];
        if a == 1 {
            C[0][0] = A.core[0][0].clone() * B.core[0][0].clone();
            let return_matrix = Matrix{core:C};
            return return_matrix;
        }
        else {
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

            let C11 = P5.plus(&P4).minus(&P2).plus(&P6);
            let C12 = P1.plus(&P2);
            let C21 = P3.plus(&P4);
            let C22 = P5.plus(&P1).minus(&P3).minus(&P7);

            for i in 0..a/2 {
                for j in 0..a/2 {
                    C[i][j] = C11.core[i][j].clone();
                    C[i][j+a/2] = C12.core[i][j].clone();
                    C[i+a/2][j] = C21.core[i][j].clone();
                    C[i+a/2][j+a/2] = C22.core[i][j].clone();
                }
            }

            let return_matrix = Matrix{core:C};

            return_matrix
        }
    }