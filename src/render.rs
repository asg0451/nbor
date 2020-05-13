use crate::planet::*;
use crate::ring_buffer::*;
use crate::vec2::*;

use terminal_size::{terminal_size, Height, Width};

use std::io::{self, Write};
use std::ops::Deref;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex}; // arc = atomic rc = atomic ref count smart ptr
use std::thread::{self, JoinHandle};
use std::time::Duration; // flush

pub struct Screen {}

impl Screen {
    pub const CURSOR_VISIBLE: &'static str = "\x1B[?25h";
    pub const CURSOR_INVISIBLE: &'static str = "\x1B[?25l";
    pub const CLEAR: &'static str = "\x1B[2J";

    pub fn term_size() -> Vec2<i32> {
        use std::convert::*;
        if let Some((Width(w), Height(h))) = terminal_size() {
            Vec2::new(w.try_into().unwrap(), h.try_into().unwrap())
        } else {
            panic!("could not get terminal size")
        }
    }

    pub fn cursor_move(x: u32, y: u32) -> String {
        format!("{}[{};{}f", 0x1B as char, y, x)
    }
}

#[derive(Debug)]
pub struct Renderer {
    breadcrumbs: RingBuffer<Vec2<f64>>,
}
impl Renderer {
    pub fn new(num_breadcrumbs: usize) -> Renderer {
        Renderer {
            breadcrumbs: RingBuffer::new(num_breadcrumbs),
        }
    }

    pub fn pretty_print_term_with_breadcrumbs(
        &mut self,
        planets: &[Planet],
        display_dimensions: Vec2<i32>,
        space_dimensions: Vec2<f64>,
    ) -> String {
        let width = display_dimensions.x() as f64;
        let height = display_dimensions.y() as f64;

        for &p in planets {
            self.breadcrumbs.push(p.loc());
        }

        let scale_x = width / space_dimensions.x();
        let scale_y = height / space_dimensions.y();

        let mut r = String::from("");
        r.reserve((planets.len() + 3) * 10);

        for crumb in self.breadcrumbs.iter() {
            let x = (crumb.x() * scale_x).floor();
            let y = (crumb.y() * scale_y).floor();

            if x > 0.0 && x < width && y > 0.0 && y < height {
                r += &(Screen::cursor_move(x as u32, y as u32) + ".")
            }
        }

        for &p in planets.iter() {
            let x = (p.loc().x() * scale_x).floor();
            let y = (p.loc().y() * scale_y).floor();

            if x < width && y < height {
                let mv = Screen::cursor_move(x as u32, y as u32);
                let s = mv + &p.id().to_string();
                r.push_str(&s);
            }
        }

        r
    }
}

pub fn render_thread(
    amx: Arc<Mutex<[Planet]>>,
    stop: Arc<AtomicBool>,
    sleep_dur: Duration,
    space_dims: Vec2<f64>,
    num_breadcrumbs: usize,
) -> JoinHandle<()> {
    thread::spawn(move || {
        let mut renderer = Renderer::new(num_breadcrumbs);

        loop {
            if stop.load(Ordering::Relaxed) {
                println!("ren stop");
                break;
            }
            {
                let mg_planets = amx.lock().unwrap();
                let str = renderer.pretty_print_term_with_breadcrumbs(
                    mg_planets.deref(),
                    Screen::term_size(),
                    space_dims,
                );
                print!("{}", Screen::CLEAR);
                print!("{}", str);
                io::stdout().flush().unwrap();
            }
            thread::sleep(sleep_dur);
        }
    })
}
