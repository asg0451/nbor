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
use std::sync::{Arc, Mutex}; // arc = atomic rc = atomic ref count smart ptr
use std::time::Duration;

fn main() {
    let p = Planet::new(10.0, Vec2::new(7.0, 5.0), Vec2::new(1.0, 0.0));
    let p2 = Planet::new(10.0, Vec2::new(4.0, 6.0), Vec2::new(0.0, -1.0));
    let planets = [p, p2]; // not mut because copy

    let space_dims = Vec2::new(10.0, 10.0);

    print!("{}", Screen::CURSOR_INVISIBLE);

    // TODO: this, properly
    ctrlc::set_handler(move || {
        println!("{}", Screen::CURSOR_VISIBLE);
        io::stdout().flush().unwrap();
        std::process::exit(1);
    })
    .unwrap();

    let ren_sleep = Duration::from_millis(100);
    let sim_sleep = Duration::from_millis(0);

    let planet_amx = Arc::new(Mutex::new(planets));
    let ren_planet_amx = Arc::clone(&planet_amx);
    let sim_planet_amx = Arc::clone(&planet_amx);
    let ren_th = render::render_thread(ren_planet_amx, ren_sleep, space_dims, 100);
    let sim_th = sim::sim_thread(sim_planet_amx, sim_sleep, 0.000_001);

    ren_th.join().unwrap();
    sim_th.join().unwrap();
}
