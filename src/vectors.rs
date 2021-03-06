
use std::ops::{Add,Sub,Mul,Div,Index,IndexMut,Range};
use float_cmp::ApproxEq;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug)]
pub struct VecN<const N: usize> {
    pub vals: [f32; N],
}


impl<const N: usize> VecN<N> {
    pub fn new() -> VecN<N> {
        VecN{vals: [0.0; N]}
    }

    pub fn project(&self, other: &VecN<N>) -> VecN<N> {
        (self * other / (other * other)) * other
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self * self)
    }

    pub fn normalize(&self) -> VecN<N> {
        self / self.length()
    }
    
    pub fn dimension() -> usize {
        N
    }

    pub fn slice(&self, slice: Range<usize>) -> &[f32] {
        &self.vals[slice]
    }

    pub fn dist(&self, other: &VecN<N>) -> f32 {
        (self.vec_to(other)).length()
    }

    pub fn vec_to(&self, other: &VecN<N>) -> VecN<N> {
        other - self
    }

}

impl VecN<2> {
    // I know this is not a real cross product but still gives perpindicular vector
    pub fn cross(&self) -> VecN<2> {
        VecN::from([self[1], -self[0]])
    }

    pub fn angle(theta: f32) -> VecN<2> {
        VecN::from([f32::cos(theta), f32::sin(theta)])
    }
}

impl VecN<3> {
    pub fn cross(&self, other: &VecN<3>) -> VecN<3> {
        let x = self[1] * other[2] - self[2] * other[1];
        let y = -(self[0] * other[2] - self[2] * other[0]);
        let z = self[0] * other[1] - self[1] * other[0];
        VecN::from([x,y,z])
    }
}

impl<const N: usize> Add<&VecN<N>> for &VecN<N> {
    type Output = VecN<N>;

    fn add(self,  other: &VecN<N>) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] + other[i];
        }
        VecN::from(answer)
    }
}
impl<const N: usize> Add<VecN<N>> for VecN<N> {
    type Output = VecN<N>;
    fn add(self,  other: VecN<N>) -> VecN<N> {
        &self + &other
    }
}
impl<const N: usize> Add<&VecN<N>> for VecN<N> {
    type Output = VecN<N>;
    fn add(self,  other: &VecN<N>) -> VecN<N> {
        &self + other
    }
}
impl<const N: usize> Add<VecN<N>> for &VecN<N> {
    type Output = VecN<N>;
    fn add(self,  other: VecN<N>) -> VecN<N> {
        self + &other
    }
}





impl<const N: usize> Sub<&VecN<N>> for &VecN<N> {
    type Output = VecN<N>;

    fn sub(self,  other: &VecN<N>) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] - other[i];
        }
        VecN::from(answer)
    }
}
impl<const N: usize> Sub<VecN<N>> for VecN<N> {
    type Output = VecN<N>;
    fn sub(self,  other: VecN<N>) -> VecN<N> {
        &self - &other
    }
}
impl<const N: usize> Sub<&VecN<N>> for VecN<N> {
    type Output = VecN<N>;
    fn sub(self,  other: &VecN<N>) -> VecN<N> {
        &self - other
    }
}
impl<const N: usize> Sub<VecN<N>> for &VecN<N> {
    type Output = VecN<N>;
    fn sub(self,  other: VecN<N>) -> VecN<N> {
        self - &other
    }
}




impl<const N: usize> Mul<&VecN<N>> for &VecN<N> {
    type Output = f32;

    fn mul(self, vector: &VecN<N>) -> f32 {
        let mut answer = 0.0;
        for i in 0..N {
            answer += self[i] * vector[i];
        }
        answer
    }
}
impl<const N: usize> Mul<VecN<N>> for &VecN<N> {
    type Output = f32;
    fn mul(self, vector: VecN<N>) -> f32 {
        self * &vector
    }
}
impl<const N: usize> Mul<&VecN<N>> for VecN<N> {
    type Output = f32;
    fn mul(self, vector: &VecN<N>) -> f32 {
        &self * vector
    }
}
impl<const N: usize> Mul<VecN<N>> for VecN<N> {
    type Output = f32;
    fn mul(self, vector: VecN<N>) -> f32 {
        self * &vector
    }
}



impl<const N: usize> Mul<&f32> for &VecN<N> {
    type Output = VecN<N>;

    fn mul(self,  scalar: &f32) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] * scalar;
        }
        VecN::from(answer)
    }

}
impl<const N: usize> Mul<f32> for VecN<N> {
    type Output = VecN<N>;
    fn mul(self,  other: f32) -> VecN<N> {
        &self * &other
    }
}
impl<const N: usize> Mul<&f32> for VecN<N> {
    type Output = VecN<N>;
    fn mul(self,  other: &f32) -> VecN<N> {
        &self * other
    }
}
impl<const N: usize> Mul<f32> for &VecN<N> {
    type Output = VecN<N>;
    fn mul(self,  other: f32) -> VecN<N> {
        self * &other
    }
}



impl<const N: usize> Mul<&VecN<N>> for &f32 {
    type Output = VecN<N>;

    fn mul(self, vector: &VecN<N>) -> VecN<N> {
        vector * self
    }
}
impl<const N: usize> Mul<VecN<N>> for &f32 {
    type Output = VecN<N>;
    fn mul(self, vector: VecN<N>) -> VecN<N> {
        vector * self
    }
}
impl<const N: usize> Mul<&VecN<N>> for f32 {
    type Output = VecN<N>;
    fn mul(self, vector: &VecN<N>) -> VecN<N> {
        vector * self   
    }
}
impl<const N: usize> Mul<VecN<N>> for f32 {
    type Output = VecN<N>;
    fn mul(self, vector: VecN<N>) -> VecN<N> {
        vector * self
    }
}




impl<const N: usize> Div<&f32> for &VecN<N> {
    type Output = VecN<N>;

    fn div(self, scalar: &f32) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] / scalar;
        }
        VecN::from(answer)
    }
}
impl<const N: usize> Div<f32> for VecN<N> {
    type Output = VecN<N>;
    fn div(self,  other: f32) -> VecN<N> {
        &self / &other
    }
}
impl<const N: usize> Div<&f32> for VecN<N> {
    type Output = VecN<N>;
    fn div(self,  other: &f32) -> VecN<N> {
        &self / other
    }
}
impl<const N: usize> Div<f32> for &VecN<N> {
    type Output = VecN<N>;
    fn div(self,  other: f32) -> VecN<N> {
        self / &other
    }
}




impl<const N: usize> Div<&VecN<N>> for &f32 {
    type Output = VecN<N>;

    fn div(self, vector: &VecN<N>) -> VecN<N> {
        vector / self
    }
}
impl<const N: usize> Div<VecN<N>> for &f32 {
    type Output = VecN<N>;
    fn div(self, vector: VecN<N>) -> VecN<N> {
        vector / self
    }
}
impl<const N: usize> Div<&VecN<N>> for f32 {
    type Output = VecN<N>;
    fn div(self, vector: &VecN<N>) -> VecN<N> {
        vector / self   
    }
}
impl<const N: usize> Div<VecN<N>> for f32 {
    type Output = VecN<N>;
    fn div(self, vector: VecN<N>) -> VecN<N> {
        vector / self
    }
}




impl<const N: usize> Index<usize> for VecN<N> {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.vals[index]
    }
}

impl<const N: usize> IndexMut<usize> for VecN<N> {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.vals[index]
    }
}

impl<const N: usize> From<[f32; N]> for VecN<N> {
    fn from(vals: [f32; N]) -> VecN<N> {
        VecN{vals}
    }
}

impl From<(f32, f32)> for VecN<2> {
    fn from(vals: (f32, f32)) -> VecN<2> {
        VecN{vals: [vals.0, vals.1]}
    }
}

impl From<(f32, f32, f32)> for VecN<3> {
    fn from(vals: (f32, f32, f32)) -> VecN<3> {
        VecN{vals: [vals.0, vals.1, vals.2]}
    }
}

impl<const N: usize> From<&Vec<f32>> for VecN<N> {
    fn from(vals: &Vec<f32>) -> VecN<N> {
        let mut new_vec = [0.0;N];
        for i in 0..vals.len() {
            new_vec[i] = vals[i];
        }
        VecN::from(new_vec)
    }
}

impl<const N: usize> PartialEq for VecN<N> {
    fn eq(&self, other: &VecN<N>) -> bool {
        for i in 0..N {
            if !self[i].approx_eq(other[i], (0.00001, 4)) {
                return false;
            }
        }
        true
    }
}

impl<const N: usize> FromIterator<f32> for VecN<N> {
    fn from_iter<I: IntoIterator<Item=f32>>(iter:I) -> Self {
        let mut vals = [0.0; N];

        let mut index = 0;
        for val in iter {
            vals[index] = val;
            index += 1;
        }
        VecN::from(vals)
    }
}

pub struct VecNIter<const N: usize> {
    vec: VecN<N>,
    at: usize,
}

impl<const N: usize> Iterator for VecNIter<N> {
    type Item = f32;
    fn next(&mut self) -> Option<f32> {
        if self.at >= N {
            return None
        }
        let ret_val = Some(self.vec[self.at]);
        self.at += 1;
        ret_val
    }
}

impl<const N: usize> IntoIterator for VecN<N> {
    type Item = f32;
    type IntoIter = VecNIter<N>;

    fn into_iter(self) -> Self::IntoIter {
        VecNIter {vec: self, at: 0}
    }

}


