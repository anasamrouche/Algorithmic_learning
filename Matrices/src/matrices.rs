use num_traits::identities::Zero;
use std::ops::{AddAssign, Mul, Sub, Range};
use std::cmp::PartialEq;

pub struct Matrix<T> {
    pub core: Vec<Vec<T>>,
}

impl<T: PartialEq> PartialEq for Matrix<T> {
    fn eq(&self, other:&Self) -> bool {
        let result: bool = true;
        let a = self.core.len();
        let b = self.core[0].len();
        let c = other.core.len();
        let d = other.core[0].len();

        if a != c || b != d {
            return false;
        }

        for i in 0..a {
            for j in 0..b {
                if self.core[i][j] != other.core[i][j] {
                    return false;
                }
            }
        }
        return true;
    }
}

impl<T: Zero + Copy + AddAssign + Sub<Output = T> + Mul<Output = T>> Matrix<T> {
    pub fn plus(&self, B:&Matrix<T>) -> Matrix<T> {
        let a:usize = self.core.len();
        let b:usize = self.core[0].len();
        let c:usize = B.core.len();
        let d:usize = B.core[0].len();

        assert!(a == c && b == d);
        
        let mut C:Vec<Vec<T>> = vec![vec![T::zero(); b]; a];
        for i in 0..a {
            for j in 0..b {
                C[i][j] = self.core[i][j] + B.core[i][j];
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
        
        let mut C:Vec<Vec<T>> = vec![vec![T::zero(); b]; a];
        for i in 0..a {
            for j in 0..b {
                C[i][j] = self.core[i][j] - B.core[i][j];
            }
        }
        let return_matrix:Matrix<T> = Matrix{core:C};
        return_matrix
    }

    pub fn get_block(&self, row_range: Range<usize>, col_range: Range<usize>) -> Matrix<T>
    {
        let C = self.core[row_range]
            .iter()
            .map(move |row| row[col_range.clone()].to_vec())
            .collect();
        Matrix{core:C}
    }

    pub fn insert_from(&mut self, B: Matrix<T>, col_start:usize, row_start: usize) {
        assert!(B.core.len() <= self.core.len() - col_start  && B.core[0].len() <= self.core[0].len() - row_start);
        for i in 0..B.core.len() {
            for j in 0..B.core[0].len() {
                self.core[col_start + i][row_start + j] = B.core[i][j];
            }
        }
    }
}