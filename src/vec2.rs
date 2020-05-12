use num::{Float, Num, Signed, Zero};
use std::ops::*;

pub trait VecContent:
    Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + AddAssign
    + Num
    + Zero
    + Copy
    + Signed
    + std::fmt::Debug
where
    Self: std::marker::Sized,
{
}
impl<T> VecContent for T where
    T: Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + AddAssign
        + Num
        + Zero
        + Copy
        + Signed
        + std::fmt::Debug
{
}

#[derive(Debug, Copy, Clone)]
pub struct Vec2<T: VecContent> {
    x: T,
    y: T,
}

impl<T: VecContent> Vec2<T> {
    pub fn zero() -> Vec2<T> {
        Vec2 {
            x: T::zero(),
            y: T::zero(),
        }
    }

    pub fn new(x: T, y: T) -> Vec2<T> {
        Vec2 { x, y }
    }

    pub fn x(&self) -> T {
        self.x
    }

    pub fn y(&self) -> T {
        self.y
    }
}

// these operations only defined when T is convertible to and from an f64
// from: we need it for sqrt
// to: because it returns a T
impl<T: VecContent + Float> Vec2<T> {
    pub fn distance(&self, o: &Self) -> T {
        ((o.x - self.x).powi(2) + (o.y - self.y).powi(2))
            .sqrt()
            .abs()
    }

    pub fn mag(&self) -> T {
        Vec2::distance(&self, &Vec2::zero())
    }

    pub fn unit(self) -> Vec2<T> {
        let m = self.mag();
        let mut v = self; // copy
        v.x = v.x / m;
        v.y = v.y / m;
        v
    }
}

impl<T: VecContent> Add for Vec2<T> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: other.x + self.x,
            y: other.y + self.y,
        }
    }
}

// scalar
impl<T: VecContent> Add<T> for Vec2<T> {
    type Output = Self;
    fn add(self, other: T) -> Self {
        Self {
            x: other + self.x,
            y: other + self.y,
        }
    }
}

impl<T: VecContent> Sub for Vec2<T> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// scalar
impl<T: VecContent> Sub<T> for Vec2<T> {
    type Output = Self;
    fn sub(self, other: T) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
        }
    }
}

impl<T: VecContent> Mul for Vec2<T> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: other.x * self.x,
            y: other.y * self.y,
        }
    }
}

// scalar
impl<T: VecContent> Mul<T> for Vec2<T> {
    type Output = Self;
    fn mul(self, other: T) -> Self {
        Self {
            x: other * self.x,
            y: other * self.y,
        }
    }
}

impl<T: VecContent> Div for Vec2<T> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

// scalar
impl<T: VecContent> Div<T> for Vec2<T> {
    type Output = Self;
    fn div(self, other: T) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
        }
    }
}

impl<T: VecContent> AddAssign for Vec2<T> {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl<T: VecContent> AddAssign<T> for Vec2<T> {
    fn add_assign(&mut self, other: T) {
        *self = *self + other;
    }
}
