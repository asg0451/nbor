mod planet {
    use crate::vec2::*;
    use std::cmp::PartialEq;

    static mut NEXT_ID: i32 = 0;

    // TODO should this be copy?
    #[derive(Clone, Copy, Debug)]
    pub struct Planet {
        id: i32,
        mass: f64,
        loc: Vec2<f64>,
        vel: Vec2<f64>,
    }

    impl Planet {
        pub fn id(&self) -> i32 {
            self.id
        }
        pub fn mass(&self) -> f64 {
            self.mass
        }
        pub fn loc(&self) -> Vec2<f64> {
            self.loc
        }

        pub fn accel_by(&mut self, accel: Vec2<f64>) {
            self.vel += accel
        }

        pub fn vel_tick(&mut self, dt: f64) {
            self.loc += self.vel * dt;
        }

        pub fn distance(&self, other: &Planet) -> f64 {
            self.loc.distance(&other.loc)
        }

        // newton
        pub fn force_between(&self, other: &Planet, g: f64) -> f64 {
            use num::traits::pow;
            let dist = other.distance(&self);
            g * self.mass * other.mass / pow(dist, 2)
        }

        pub fn vector_between(&self, other: &Planet) -> Vec2<f64> {
            other.loc - self.loc
        }

        pub fn new(mass: f64, loc: Vec2<f64>, vel: Vec2<f64>) -> Planet {
            let id = unsafe { Planet::next_id() };
            Planet {
                mass,
                loc,
                vel,
                id: id,
            }
        }

        unsafe fn next_id() -> i32 {
            let id = NEXT_ID;
            NEXT_ID += 1;
            id
        }
    }

    impl PartialEq for Planet {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }
}

pub use planet::*;
