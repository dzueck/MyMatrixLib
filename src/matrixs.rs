use std::ops::{Mul,Div,Index,IndexMut};
use crate::vectors::VecN;

#[derive(Clone, Debug)]
pub struct Mat<const N: usize, const M: usize> {
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
            mat[i][i] = 1.0;
        }
        mat
    }
    
    pub fn det(&self) -> f32 {
        determinent(self.into())  
    }

}

impl Mat<2,2> {
    pub fn rotation(theta: f32) -> Mat<2,2> {
        let cos = f32::cos(theta);
        let sin = f32::sin(theta);
        Mat::from([[cos, -sin],
                    [sin, cos]])
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

/*impl<T, const N: usize, const M: usize> From<T> for Mat<N, M> 
    where T: Index<usize, Output = dyn Index<usize, Output = f32>> {

    fn from(vals: T) -> Self {
        let mut mat = Mat::new();
        for i in 0..N {
            for j in 0..M {
                mat[i][j] = vals[i][j];
            }
        }
        mat
    }
}*/



impl<const N: usize, const M: usize> Mul<&f32> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, scalar: &f32) -> Mat<N, M> {
        let mut new_mat = [[0.0; M]; N];
        for i in 0..N {
            for j in 0..M {
                new_mat[i][j] = self.vals[i][j] * scalar;
            }
        }
        Mat::from(new_mat)
    }
}

impl<const N: usize, const M: usize> Mul<f32> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, scalar: f32) -> Mat<N, M> {
        self * &scalar
    }
}

impl<const N: usize, const M: usize> Mul<&f32> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, scalar: &f32) -> Mat<N, M> {
        &self * scalar
    }
}



impl<const N: usize, const M: usize> Div<&f32> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn div(self, scalar: &f32) -> Mat<N, M> {
        let mut new_mat = [[0.0; M]; N];
        for i in 0..N {
            for j in 0..M {
                new_mat[i][j] = self.vals[i][j] / scalar;
            }
        }
        Mat::from(new_mat)
    }
}

impl<const N: usize, const M: usize> Div<f32> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn div(self, scalar: f32) -> Mat<N, M> {
        self / &scalar
    }
}

impl<const N: usize, const M: usize> Div<&f32> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn div(self, scalar: &f32) -> Mat<N, M> {
        &self / scalar
    }
}

impl<const N: usize, const M: usize> Div<f32> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn div(self, scalar: f32) -> Mat<N, M> {
        &self / &scalar
    }
}


impl<const N: usize, const M: usize> Mul<f32> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, scalar: f32) -> Mat<N, M> {
        &self * &scalar
    }
}


impl<const N: usize, const M: usize, const Z: usize> Mul<&Mat<M, Z>> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, other: &Mat<M, Z>) -> Mat<N, M> {
        let mut new_mat = Mat::new();
        let other_t = other.transpose();
        for i in 0..N {
            for j in 0..Z {
                new_mat[i][j] = self[i] * &other_t[j];
            }
        }
        new_mat
    }
}

impl<const N: usize, const M: usize, const Z: usize> Mul<&Mat<M, Z>> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, other: &Mat<M, Z>) -> Mat<N, M> {
        &self * other
    }
}

impl<const N: usize, const M: usize, const Z: usize> Mul<Mat<M, Z>> for Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, other: Mat<M, Z>) -> Mat<N, M> {
        &self * &other
    }
}

impl<const N: usize, const M: usize, const Z: usize> Mul<Mat<M, Z>> for &Mat<N, M> {
    type Output = Mat<N, M>;

    fn mul(self, other: Mat<M, Z>) -> Mat<N, M> {
        self * &other
    }
}



impl<const N: usize, const M: usize> Mul<&VecN<M>> for &Mat<N, M> {
    type Output = VecN<N>;

    fn mul(self, other: &VecN<M>) -> VecN<N> {
        let mut new_vec = VecN::new();
        for i in 0..N {
            new_vec[i] = self[i] * other;
        }
        new_vec
    }
}

impl<const N: usize, const M: usize> Mul<&VecN<M>> for Mat<N, M> {
    type Output = VecN<N>;

    fn mul(self, other: &VecN<M>) -> VecN<N> {
        &self * other
    }
}

impl<const N: usize, const M: usize> Mul<VecN<M>> for Mat<N, M> {
    type Output = VecN<N>;

    fn mul(self, other: VecN<M>) -> VecN<N> {
        &self * &other
    }
}

impl<const N: usize, const M: usize> Mul<VecN<M>> for &Mat<N, M> {
    type Output = VecN<N>;

    fn mul(self, other: VecN<M>) -> VecN<N> {
        self * &other
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
        let mut vals: Vec<Vec<f32>> = Vec::with_capacity(N);

        for i in 0..N {
            vals.push(mat[i].into_iter().collect());
        }
        vals
    }
}

impl<const N: usize, const M: usize> PartialEq for Mat<N, M> {
    fn eq(&self, other: &Mat<N, M>) -> bool {
        for i in 0..N {
            if !(self[i] == other[i]) {
                return false;
            }
        }
        true
    }
}



pub fn determinent(vals: Vec<Vec<f32>>) -> f32 {
    if vals.len() == 1 {
        return vals[0][0];
    }
    
    let mut answer = 0.0;

    let top_vals = &vals[0];
    let bottom_vals = &vals[1..vals.len()];

    for i in 0..vals.len() {
        // i is the column of the top row num
        let top_val = top_vals[i];
        let mut new_det = Vec::<Vec<f32>>::new();
        for j in 0..bottom_vals.len() {
            let mut new_row = Vec::new();
            new_row.extend_from_slice(&bottom_vals[j][0..i]);
            new_row.extend_from_slice(&bottom_vals[j][(i+1)..vals.len()]);
            new_det.push(new_row);
        }

        answer += top_val * determinent(new_det) * ((i % 2) as f32 - 0.5) * -2.0;

    }
    answer
}
