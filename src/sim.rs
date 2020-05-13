use crate::planet::*;
use crate::vec2::*;

use std::ops::DerefMut;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex}; // arc = atomic rc = atomic ref count smart ptr
use std::thread::{self, JoinHandle};
use std::time::Duration;

// mutable state during ticking -- purely as optimization
pub struct Simulator {
    accels: Vec<Vec2<f64>>,
}

impl Simulator {
    pub fn new(num_planets: usize) -> Self {
        Simulator {
            accels: vec![Vec2::zero(); num_planets],
        }
    }

    pub fn tick(&mut self, dt: f64, g: f64, planets: &mut [Planet]) {
        // apply velocities
        for p in planets.iter_mut() {
            (*p).vel_tick(dt);
        }

        for &p1 in planets.iter() {
            // zero out accel vector for each planet before considering it
            self.accels[p1.id() as usize] = Vec2::zero();

            for &p2 in planets.iter() {
                if p1 == p2 {
                    continue;
                }

                let f_s = p1.force_between(&p2, g);
                let fv = p1.vector_between(&p2).unit() * f_s;
                let mut cur_a = self.accels[p1.id() as usize];
                cur_a = fv / p1.mass() + cur_a;
                self.accels[p1.id() as usize] = cur_a;
            }
        }

        for p in planets.iter_mut() {
            let v = self.accels[p.id() as usize] * dt;
            (*p).accel_by(v);
        }
    }
}

pub fn sim_thread(
    amx: Arc<Mutex<[Planet]>>,
    stop: Arc<AtomicBool>,
    sleep_dur: Duration,
    dt: f64,
) -> JoinHandle<()> {
    let num_planets: usize;
    {
        num_planets = amx.lock().unwrap().len();
    }
    let mut sim = Simulator::new(num_planets);

    thread::spawn(move || {
        loop {
            if stop.load(Ordering::Relaxed) {
                println!("sim stop");
                break;
            }
            {
                let mut mg_planets = amx.lock().unwrap();
                // i think this is required to be explicitly called
                // otherwise since array is copy it'll copy. same in ren thread
                sim.tick(dt, 1.0, mg_planets.deref_mut());
            }
            thread::sleep(sleep_dur);
        }
    })
}
