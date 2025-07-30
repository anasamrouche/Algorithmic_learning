use num_traits::identities::Zero;
use std::ops::{AddAssign, Mul, Sub, Range};

#[derive(Debug)]
pub struct Matrix<T> {
    pub core: Vec<Vec<T>>,
}

impl<T: Zero + Clone + AddAssign + Sub<Output = T> + Mul<Output = T>> Matrix<T> {
    pub fn plus(&self, B:&Matrix<T>) -> Matrix<T> {
        let a:usize = self.core.len();
        let b:usize = self.core[0].len();
        let c:usize = B.core.len();
        let d:usize = B.core[0].len();

        assert!(a == c && b == d);
        
        let mut C:Vec<Vec<T>> = vec![vec![T::zero().clone(); b]; a];
        for i in 0..a {
            for j in 0..b {
                C[i][j] = self.core[i][j].clone() + B.core[i][j].clone();
            }
        }
        let return_matrix:Matrix<T> = Matrix{core:C};
        return_matrix
    }

    pub fn minus(&self, B:&Matrix<T>) -> Matrix<T> {
        let a:usize = self.core.len();
        let b:usize = self.core[0].len();
        let c:usize = B.core.len();
        let d:usize = B.core[0].len();

        assert!(a == c && b == d);
        
        let mut C:Vec<Vec<T>> = vec![vec![T::zero().clone(); b]; a];
        for i in 0..a {
            for j in 0..b {
                C[i][j] = self.core[i][j].clone() - B.core[i][j].clone();
            }
        }
        let return_matrix:Matrix<T> = Matrix{core:C};
        return_matrix
    }

    pub fn get_block(&self, row_range: Range<usize>, col_range: Range<usize>) -> Matrix<T>
    {
        let C = self.core[row_range]
            .iter()
            .map(|row| row[col_range.clone()].to_vec())
            .collect();
        Matrix{core:C}
    }
}