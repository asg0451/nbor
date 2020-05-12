mod sim {
    use crate::planet::*;
    use crate::vec2::*;

    use std::ops::DerefMut;
    use std::sync::{Arc, Mutex}; // arc = atomic rc = atomic ref count smart ptr
    use std::thread::{self, JoinHandle};
    use std::time::Duration;

    pub fn tick(dt: f64, g: f64, planets: &mut [Planet]) {
        // apply velocities
        for p in planets.iter_mut() {
            (*p).vel_tick(dt);
        }

        let mut accels = vec![Vec2::zero(); planets.len()];

        for &p1 in planets.iter() {
            for &p2 in planets.iter() {
                if p1 == p2 {
                    continue;
                }

                let f_s = p1.force_between(&p2, g);
                let fv = p1.vector_between(&p2).unit() * f_s;
                let mut cur_a = accels[p1.id() as usize];
                cur_a = fv / p1.mass() + cur_a;
                accels[p1.id() as usize] = cur_a;
            }
        }

        for p in planets.iter_mut() {
            let v = accels[p.id() as usize] * dt;
            (*p).accel_by(v);
        }
    }

    pub fn sim_thread(amx: Arc<Mutex<[Planet]>>, sleep_dur: Duration) -> JoinHandle<()> {
        thread::spawn(move || loop {
            loop {
                {
                    let mut mg_planets = amx.lock().unwrap();
                    // i think this is required to be explicitly called
                    // otherwise since array is copy it'll copy. same in ren thread
                    tick(0.000001, 1.0, mg_planets.deref_mut());
                }
                thread::sleep(sleep_dur);
            }
        })
    }
}

pub use sim::*;
