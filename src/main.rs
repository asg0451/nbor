mod planet;
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

    let stats = Arc::new(Mutex::new(stats::Stats::new()));

    // TODO?: move all arcs into one acr, clone that
    let ren_th = render::render_thread(
        planet_amx.clone(),
        stop.clone(),
        stats.clone(),
        ren_sleep,
        space_dims,
        100,
    );
    let sim_th = sim::sim_thread(
        planet_amx.clone(),
        stop.clone(),
        stats.clone(),
        sim_sleep,
        0.0000001,
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
        main_stop = cvar.wait(main_stop).unwrap();
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
}
