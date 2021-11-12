// 178

use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Vec2 {
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
        f32::sqrt(self.x * self.x + self.y * self.y)
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

