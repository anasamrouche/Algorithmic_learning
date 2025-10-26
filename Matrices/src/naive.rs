use crate::matrices::Matrix;
use std::ops::{AddAssign, Mul};
use num_traits::Zero;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::marker::Sync;

pub fn multiply_naive<T:Zero + Copy + AddAssign + Mul<Output = T> + Send + Sync + 'static> (A:&Matrix<T>, B:&Matrix<T>) -> Matrix<T> {
        let a: usize = A.core.len();
        let b: usize = B.core[0].len();
        let a_rows: usize = A.core[0].len();
        let b_columns: usize = B.core.len();
        assert!(a_rows == b_columns); 
        let mut C:Arc<Vec<Mutex<Vec<T>>>> = Arc::new((0..a).map(|_| Mutex::new(vec![T::zero(); b])).collect());
        let A_arc = Arc::new(A.clone());
        let B_arc = Arc::new(B.clone());

        let mut handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(a);

        for i in 0..a {
            let A_clone = Arc::clone(&A_arc);
            let B_clone = Arc::clone(&B_arc);
            let C_clone = Arc::clone(&C);

            let handle = thread::spawn(move || {
                let mut row = vec![T::zero(); b];
                for j in 0..b {
                    for k in 0..a_rows {
                        row[j] += A_clone.core[i][k] * B_clone.core[k][j];
                    }
                }
                let mut line = C_clone[i].lock().unwrap();
                *line = row;
            });
            handles.push(handle);
        };

        for handle in handles {
            handle.join().unwrap();
        }

        let mut result = Vec::with_capacity(a);
        for i in 0..a {
            let line = C[i].lock().unwrap();
            result.push(line.clone());
        }

        let return_mat:Matrix<T> = Matrix{core:result};
        return_mat
    }