
use std::ops::{Add,Sub,Mul,Div,Index,IndexMut};
use float_cmp::ApproxEq;
use std::iter::FromIterator;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
    pub fn new() -> Vec2 {
        Vec2::from(0.0, 0.0)
    }

    pub fn from(x: f32, y: f32) -> Vec2 {
        Vec2{x,y}
    }

    pub fn dot(self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    pub fn project(self, other: Vec2) -> Vec2 {
        (self.dot(other) / other.dot(other)) * other
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }
    
    // Note this is not a reall cross product but still gives perpindicular vector
    pub fn cross(self) -> Vec2 {
        Vec2 {x: self.y, y: -self.x}
    }
}

impl Add for Vec2 {
    type Output = Self;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x + other.x, y: self.y + other.y}
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { x: self.x - other.x, y: self.y - other.y}
    }
}

impl Mul<f32> for Vec2 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x * scalar, y: self.y * scalar}
    }
}

impl Mul<Vec2> for f32 {
    type Output = Vec2;

    fn mul(self, vector: Vec2) -> Vec2 {
        Vec2 { x: vector.x * self, y: vector.y * self}
    }
}

impl Div<f32> for Vec2 {
    type Output = Self;

    fn div(self, scalar: f32) -> Vec2 {
        Vec2 { x: self.x / scalar, y: self.y / scalar}
    }
}

impl Div<Vec2> for f32 {
    type Output = Vec2;

    fn div(self, vector: Vec2) -> Vec2 {
        Vec2 { x: vector.x / self, y: vector.y / self}
    }
}

impl Index<usize> for Vec2 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index {} out of vector bounds", index)
        }
    }
}

impl IndexMut<usize> for Vec2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index {} out of vector bounds", index)
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(vals: [f32; 2]) -> Self {
        Vec2::from(vals[0], vals[1])
    }
}

impl From<VecN<2>> for Vec2 {
    fn from(v: VecN<2>) -> Self {
        Vec2::from(v[0], v[1])
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
       self.x.approx_eq(other.x, (0.00001, 4)) && self.y.approx_eq(other.y, (0.00001, 4))
    }
}



#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new() -> Vec3 {
        Vec3{x: 0.0, y: 0.0, z: 0.0}
    }

    pub fn from(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3{x,y,z}
    }

    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn project(self, other: Vec3) -> Vec3 {
        (self.dot(other) / other.dot(other)) * other
    }

    pub fn length(self) -> f32 {
        f32::sqrt(self.dot(self))
    }
    
    // Note this is not a reall cross product but still gives perpindicular vector
    pub fn cross(self, other: Vec3) -> Vec3 {
        Vec3 {x: self.y * other.z - self.z * other.y,
              y: self.x * other.z - self.z * other.x,
              z: self.x * other.y - self.y * other.x,}
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x + other.x, y: self.y + other.y, z: self.z + other.z}
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x - other.x, y: self.y - other.y, z: self.z - other.z}
    }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x * scalar, y: self.y * scalar, z: self.z * scalar}
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vector: Vec3) -> Vec3 {
        Vec3 { x: vector.x * self, y: vector.y * self, z: vector.z * self}
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 { x: self.x / scalar, y: self.y / scalar, z: self.z / scalar}
    }
}

impl Div<Vec3> for f32 {
    type Output = Vec3;

    fn div(self, vector: Vec3) -> Vec3 {
        Vec3 { x: vector.x / self, y: vector.y / self, z: vector.z / self}
    }
}

impl From<Vec2> for Vec3 {
    fn from(v2: Vec2) -> Self {
        Vec3 {x: v2.x, y: v2.y, z: 0.0}
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index {} out of vector bounds", index)
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index {} out of vector bounds", index)
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(vals: [f32; 3]) -> Self {
        Vec3::from(vals[0], vals[1], vals[2])
    }
}

impl From<VecN<3>> for Vec3 {
    fn from(v: VecN<3>) -> Self {
        Vec3::from(v[0], v[1], v[2])
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
       self.x.approx_eq(other.x, (0.00001, 4)) && self.y.approx_eq(other.y, (0.00001, 4)) && self.z.approx_eq(other.z, (0.00001, 4))
    }
}




#[derive(Clone, Copy, Debug)]
pub struct VecN<const N: usize> {
    pub vals: [f32; N],
}

impl<const N: usize> VecN<N> {
    pub fn new() -> VecN<N> {
        VecN{vals: [0.0; N]}
    }

    pub fn dot(&self, other: &VecN<N>) -> f32 {
        let mut answer = 0.0;
        for i in 0..N {
            answer += self[i] * other[i];
        }
        answer
    }

    pub fn project(&self, other: &VecN<N>) -> VecN<N> {
        (self.dot(other) / other.dot(other)) * other
    }

    pub fn length(&self) -> f32 {
        f32::sqrt(self.dot(self))
    }
    
    pub fn dimension(&self) -> usize {
        N
    }
}

impl<const N: usize> Add for &VecN<N> {
    type Output = VecN<N>;

    fn add(self,  other: &VecN<N>) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] + other[i];
        }
        VecN::from(answer)
    }
}

impl<const N: usize> Sub for &VecN<N> {
    type Output = VecN<N>;

    fn sub(self,  other: &VecN<N>) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] - other[i];
        }
        VecN::from(answer)
    }
}

impl<const N: usize> Mul<f32> for &VecN<N> {
    type Output = VecN<N>;

    fn mul(self,  scalar: f32) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] * scalar;
        }
        VecN::from(answer)
    }

}

impl<const N: usize> Mul<&VecN<N>> for f32 {
    type Output = VecN<N>;

    fn mul(self, vector: &VecN<N>) -> VecN<N> {
        vector * self
    }
}

impl<const N: usize> Div<f32> for &VecN<N> {
    type Output = VecN<N>;

    fn div(self, scalar: f32) -> VecN<N> {
        let mut answer = [0.0; N];
        for i in 0..N {
            answer[i] = self[i] / scalar;
        }
        VecN::from(answer)
    }
}

impl<const N: usize> Div<&VecN<N>> for f32 {
    type Output = VecN<N>;

    fn div(self, vector: &VecN<N>) -> VecN<N> {
        vector * self
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

impl<const N: usize> From<Vec2> for VecN<N> {
    fn from(v2: Vec2) -> Self {
        let mut new_vec = [0.0;N];
        new_vec[0] = v2.x;
        new_vec[1] = v2.y;
        VecN::from(new_vec)
    }
}

impl<const N: usize> From<Vec3> for VecN<N> {
    fn from(v3: Vec3) -> Self {
        let mut new_vec = [0.0;N];
        new_vec[0] = v3.x;
        new_vec[1] = v3.y;
        new_vec[2] = v3.z;
        VecN::from(new_vec)
    }
}

impl<const N: usize> From<[f32; N]> for VecN<N> {
    fn from(vals: [f32; N]) -> VecN<N> {
        VecN{vals}
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
        }
        VecN::from(vals)
    }
}


