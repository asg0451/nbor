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

use std::ops::{Deref, DerefMut};

use std::sync::{Arc, Mutex}; // arc = atomic rc = atomic ref count smart ptr
use std::thread;
use std::time::Duration;

fn main() {
    let p = Planet::new(10.0, Vec2::new(7.0, 5.0), Vec2::new(1.0, 0.0));
    let p2 = Planet::new(10.0, Vec2::new(4.0, 6.0), Vec2::zero());
    let mut planets = [p, p2];

    let space_dim = Vec2::new(10.0, 10.0);

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

    let ren_th = thread::spawn(move || {
        // TODO: render, sim threads
        let mut renderer = render::Renderer::new(2_000);

        loop {
            {
                let mg_planets = ren_planet_amx.lock().unwrap();
                let str = renderer.pretty_print_term_with_breadcrumbs(
                    mg_planets.deref(),
                    &Screen::term_size(),
                    &space_dim,
                );
                print!("{}", Screen::CLEAR);
                print!("{}", str);
                io::stdout().flush().unwrap();
            }
            thread::sleep(ren_sleep);
        }
    });

    let sim_th = thread::spawn(move || loop {
        loop {
            {
                let mut mg_planets = sim_planet_amx.lock().unwrap();
                // i think this is required to be explicitly called otherwise since array is copy it'll copy. same ^
                sim::tick(0.000001, 1.0, mg_planets.deref_mut());
            }
            thread::sleep(sim_sleep);
        }
    });

    ren_th.join().unwrap();
    sim_th.join().unwrap();
}
