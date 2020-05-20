mod planet;
mod render;
mod ring_buffer;
mod sim;
mod stats;
mod threader;
mod util;
mod vec2;

use planet::*;
use render::*;
use vec2::*;

use std::io::{self, Write}; // flush
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Condvar, Mutex,
}; // arc = atomic rc = atomic ref count smart ptr
use std::thread;
use std::time::Duration;

fn main() {
    // {
    //     let t = threader::Threader::new(|| loop {
    //         println!("act!");
    //         thread::sleep(Duration::from_millis(100));
    //     });
    //     thread::sleep(Duration::from_millis(1000));
    // }

    // println!("descope");

    // return;

    // TODO: look into https://nalgebra.org/vectors_and_matrices/
    let p = Planet::new(10.0, Vec2::new(7.0, 5.0), Vec2::new(1.0, 0.0));
    let p2 = Planet::new(10.0, Vec2::new(4.0, 6.0), Vec2::new(0.0, -1.0));
    let planets = [p, p2]; // not mut because copy

    let space_dims = Vec2::new(10.0, 10.0);

    print!("{}", Screen::CURSOR_INVISIBLE);

    let ren_sleep = Duration::from_millis(100);
    let sim_sleep = Duration::from_millis(0);
    let planet_amx = Arc::new(Mutex::new(planets));

    let stats = Arc::new(Mutex::new(stats::Stats::new()));

    {
        // TODO: drop never called. i think because its in an arc with one reference in the
        // threader's thread itself
        // i think the threader is a bad idea. join in drop not amazing
        // need some abstraction though. TODO: find a better one

        let _ren_th = render::ren_threader(
            planet_amx.clone(),
            stats.clone(),
            ren_sleep,
            space_dims,
            100,
        );
        let _sim_th = sim::sim_threader(planet_amx.clone(), stats.clone(), sim_sleep, 0.000001);

        let stop_pair = Arc::new((Mutex::new(false), Condvar::new()));
        {
            let stop_pair = stop_pair.clone();
            ctrlc::set_handler(move || {
                println!("caught cc");
                let (lock, cvar) = &*stop_pair;
                let mut stop = lock.lock().unwrap();
                *stop = true;
                cvar.notify_one();
            })
            .unwrap();
        }

        let (lock, cvar) = &*stop_pair;
        let mut stop = lock.lock().unwrap();
        while !*stop {
            stop = cvar.wait(stop).unwrap();
        }
        println!("stopping");
    }
    println!("{} {}", Screen::CURSOR_VISIBLE, Screen::CLEAR);
    io::stdout().flush().unwrap();
    thread::sleep(Duration::from_secs(2));
}
