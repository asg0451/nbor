mod vec2 {
    use num::{Num, Zero};
    use std::ops::*;

    pub trait VecContent:
        Add<Output = Self>
        + Sub<Output = Self>
        + Mul<Output = Self>
        + Div<Output = Self>
        + Num
        + Zero
        + Copy
    where
        Self: std::marker::Sized,
    {
    }
    impl<T> VecContent for T where
        T: Add<Output = T>
            + Sub<Output = T>
            + Mul<Output = T>
            + Div<Output = T>
            + Num
            + Zero
            + Copy
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

    // scalar add
    impl<T: VecContent> Add<T> for Vec2<T> {
        type Output = Self;
        fn add(self, other: T) -> Self {
            Self {
                x: other + self.x,
                y: other + self.y,
            }
        }
    }
}

pub use vec2::*;
