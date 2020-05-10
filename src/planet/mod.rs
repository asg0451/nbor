mod planet {
    use crate::vec2::*;

    static mut NEXT_ID: i32 = 0;

    #[derive(Clone, Copy, Debug)]
    pub struct Planet {
        id: i32,
        mass: f64,
        loc: Vec2<f64>,
        vel: Vec2<f64>,
    }

    impl Planet {
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
            NEXT_ID += 1;
            NEXT_ID
        }
    }
}

pub use planet::*;
