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

fn main() {
    // println!("{}", Screen::clear());
    // println!("screen size: {:?}", Screen::term_size());
    // print!("{}", Screen::cursor_move(20, 10));
    // println!("moved?");

    let p = Planet::new(10.0, Vec2::new(7.0, 5.0), Vec2::new(1.0, 0.0));
    let p2 = Planet::new(10.0, Vec2::new(4.0, 6.0), Vec2::zero());
    let mut planets = [p, p2];
    let space_dim = Vec2::new(10.0, 10.0);

    // let mut rb = ring_buffer::RingBuffer::<i32>::new(10);
    // for i in 0..15 {
    //     rb.push(i);
    //     println!("added {:?} to {:?}", i, rb);
    // }
    // println!("{:?}", rb);
    // for el in rb.iter() {
    //     println!("{:?}", el);
    // }

    // let mut rb = ring_buffer::RingBuffer::<i32>::new(10);
    // for i in 0..5 {
    //     rb.push(i);
    //     println!("added {:?} to {:?}", i, rb);
    // }
    // println!("{:?}", rb);
    // for el in rb.iter() {
    //     println!("{:?}", el);
    // }

    print!("{}", Screen::CURSOR_INVISIBLE);

    // TODO: this, properly
    ctrlc::set_handler(move || {
        println!("{}", Screen::CURSOR_VISIBLE);
        io::stdout().flush().unwrap();
        std::process::exit(1);
    })
    .unwrap();

    // TODO: render, sim threads
    let renderer = &mut render::Renderer::new(2_000);

    loop {
        sim::tick(0.001, 1.0, &mut planets);
        let str =
            renderer.pretty_print_term_with_breadcrumbs(&planets, &Screen::term_size(), &space_dim);
        print!("{}", Screen::CLEAR);
        print!("{}", str);
        io::stdout().flush().unwrap();

        // std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
