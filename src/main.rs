mod planet;
mod real_data;
mod render;
mod ring_buffer;
mod sim;
mod stats;
mod util;
mod vec2;

use planet::*;
use render::*;
use vec2::*;

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
}; // arc = atomic rc = atomic ref count smart ptr
use std::time::Duration;

// arg parsing
extern crate clap;
use clap::{App, Arg};

// TODO: look into https://nalgebra.org/vectors_and_matrices/
fn main() {
    let matches = App::new("nbor")
        .arg(
            Arg::with_name("dt")
                .short("d")
                .long("dt")
                .takes_value(true)
                .help("time delta"),
        )
        .get_matches();
    let dt = matches
        .value_of("dt")
        .unwrap_or("1.0")
        .parse::<f64>()
        .expect("invalid value for dt");

    let space_dims = Vec2::new(
        1000.0 * real_data::NASA_RADIUS_FACTOR,
        500.0 * real_data::NASA_RADIUS_FACTOR,
    );
    let sun: Planet = Planet::new(real_data::MASS_OF_SUN, space_dims / 2.0, Vec2::zero());
    let mut planets = vec![sun];
    planets.append(&mut real_data::real_planets(sun));

    print!("{}", Screen::CURSOR_INVISIBLE);

    let ren_sleep = Duration::from_millis(100);
    let sim_sleep = Duration::from_millis(0);
    let stop = Arc::new(AtomicBool::new(false));
    let planet_amx = Arc::new(Mutex::new(planets));

    let stats = Arc::new(Mutex::new(stats::Stats::new()));

    // TODO?: move all arcs into one arc, clone that
    let ren_th = render::render_thread(
        planet_amx.clone(),
        stop.clone(),
        stats.clone(),
        ren_sleep,
        space_dims,
        1000,
    );
    let sim_th = sim::sim_thread(
        planet_amx.clone(),
        stop.clone(),
        stats.clone(),
        sim_sleep,
        dt,
        sim::Simulator::G_REAL,
    );

    let main_stop_pair = Arc::new((Mutex::new(false), Condvar::new()));
    {
        let main_stop_pair = main_stop_pair.clone();
        ctrlc::set_handler(move || {
            println!("caught cc");
            let (lock, cvar) = &*main_stop_pair; // deref arc to unwrap, ref because we want a ref
            let mut stop = lock.lock().unwrap();
            *stop = true;
            cvar.notify_one();
        })
        .unwrap();
    }

    let (lock, cvar) = &*main_stop_pair;
    let mut main_stop = lock.lock().unwrap();
    while !*main_stop {
        let (ml, timeout_res) = cvar
            .wait_timeout(main_stop, Duration::from_secs(10))
            .unwrap();
        main_stop = ml;
        if timeout_res.timed_out() {
            println!("\ntimeout");
            break;
        }
    }
    println!("{}\n", Screen::CURSOR_VISIBLE);
    println!("stopping");
    stop.store(true, Ordering::Relaxed);

    ren_th.join().unwrap();
    sim_th.join().unwrap();

    let mut stats = stats.lock().unwrap();
    stats
        .dump(std::path::Path::new("durations.txt"))
        .expect("failed to dump");
    println!("dumped durations")
}
