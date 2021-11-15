use std::ops::{Add,Sub,Mul,Div,Index,IndexMut};
use float_cmp::ApproxEq;
use crate::vectors::VecN;

struct Mat<const N: usize, const M: usize> {
    vals: [VecN<M>; N],
}

impl<const N: usize, const M: usize> Mat<N, M> {
    pub fn new() -> Mat<N, M> {
        Mat {vals: [VecN::<M>::new(); N]}
    }

    pub fn transpose(&self) -> Mat<M, N> {
        let mut new_mat = Mat::<M, N>::new();
        for i in 0..N {
            for j in 0..M {
                new_mat[j][i] = self[i][j];
            }
        }
        new_mat
    }

}

impl<const N:usize> Mat<N, N> {
    pub fn identity() -> Mat<N, N> {
        let mut mat = Self::new();
        for i in 0..N {
            mat[i][i] = 0.0;
        }
        mat
    }
    
    pub fn det(&self) -> f32 {
        Self::determinent(self.into())  
    }

    fn determinent(vals: Vec<Vec<f32>>) -> f32 {
        if vals.len() == 1 {
            return vals[0][0];
        }
        
        let mut answer = 0.0;

        let top_vals = &vals[0];
        let bottom_vals = &vals[1..vals.len()];

        for z in 0..top_vals.len() { 
            let mut new_det = Vec::<Vec<f32>>::new();
            for i in 0..bottom_vals.len() {
                for j in 0..vals.len() {
                    new_det[i].extend_from_slice(&bottom_vals[i][0..j]);
                    new_det[i].extend_from_slice(&bottom_vals[i][j..vals.len()]);
                }
            }
            answer += top_vals[z] * Self::determinent(new_det) * ((z % 2) as f32 - 0.5) * 2.0;
        }
        answer
    }
}

impl<const N: usize, const M: usize> Index<usize> for Mat<N, M> {
    type Output = VecN<M>;

    fn index(&self, index: usize) -> &VecN<M> {
        &self.vals[index]
    }
}

impl<const N: usize, const M: usize> IndexMut<usize> for Mat<N, M> {
    fn index_mut(&mut self, index: usize) -> &mut VecN<M> {
        &mut self.vals[index]
    }
}

impl<const N: usize, const M: usize> From<[[f32; M]; N]> for Mat<N, M> {
    fn from(vals: [[f32; M]; N]) -> Self {
        let mut new_mat = Self::new();
        for i in 0..N {
            new_mat[i] = vals[i].into();
        }
        new_mat
    }
}

impl<const N: usize, const M: usize> From<[VecN<M>; N]> for Mat<N, M> {
    fn from(vals: [VecN<M>; N]) -> Self {
        Mat {vals}
    }
}

impl<const N: usize, const M: usize> Mul<f32> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, scalar: f32) -> Mat<N, M> {
        let mut new_mat = [[0.0; M]; N];
        for i in 0..N {
            for j in 0..M {
                new_mat[i][j] = self.vals[i][j] * scalar;
            }
        }
        Mat::from(new_mat)
    }
}

impl<const N: usize, const M: usize, const Z: usize> Mul<&Mat<M, Z>> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, other: &Mat<M, Z>) -> Mat<N, M> {
        let mut new_mat = Mat::new();
        let other_t = other.transpose();
        for i in 0..M {
            for j in 0..M {
                new_mat[i][j] = self[i].dot(&other_t[j]);
            }
        }
        new_mat
    }
}




impl<const N: usize, const M: usize> From<&Vec<&Vec<f32>>> for Mat<N, M> {
    fn from(vals: &Vec<&Vec<f32>>) -> Self {
        let mut mat = Mat::new();

        for i in 0..vals.len() {
            mat[i] = vals[i].into();
        }

        mat
    }
}

impl<const N: usize, const M: usize> From<&Mat<N, M>> for Vec<Vec<f32>> {
    fn from(mat: &Mat<N, M>) -> Self {
        let mut vals: Vec<Vec<f32>> = Vec::new();

        for i in 0..N {
            vals[i] = mat[i].into_iter().collect();
        }

        vals
    }
}
