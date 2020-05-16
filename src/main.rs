mod planet;
mod render;
mod ring_buffer;
mod sim;
mod util;
mod vec2;

use planet::*;
use render::*;
use vec2::*;

use std::io::{self, Write}; // flush
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
}; // arc = atomic rc = atomic ref count smart ptr
use std::thread;
use std::time::Duration;

fn main() {
    // TODO: look into https://nalgebra.org/vectors_and_matrices/
    let p = Planet::new(10.0, Vec2::new(7.0, 5.0), Vec2::new(1.0, 0.0));
    let p2 = Planet::new(10.0, Vec2::new(4.0, 6.0), Vec2::new(0.0, -1.0));
    let planets = [p, p2]; // not mut because copy

    let space_dims = Vec2::new(10.0, 10.0);

    print!("{}", Screen::CURSOR_INVISIBLE);

    let ren_sleep = Duration::from_millis(100);
    let sim_sleep = Duration::from_millis(0);

    let stop = Arc::new(AtomicBool::new(false));

    let planet_amx = Arc::new(Mutex::new(planets));

    let ren_th =
        render::render_thread(planet_amx.clone(), stop.clone(), ren_sleep, space_dims, 100);
    let sim_th = sim::sim_thread(planet_amx.clone(), stop.clone(), sim_sleep, 0.0000001);

    {
        let stop = stop.clone();
        ctrlc::set_handler(move || {
            println!("{} {}", Screen::CURSOR_VISIBLE, Screen::CLEAR);
            stop.store(true, Ordering::Relaxed);
            io::stdout().flush().unwrap();

            // TODO: can't get the join handles into here. i want to join on them here but like
            // do i really care
            thread::sleep(Duration::from_millis(1000));
            std::process::exit(1);
        })
        .unwrap();
    }
    ren_th.join().unwrap();
    sim_th.join().unwrap();
}
