mod planet;
mod sim;
mod util;
mod vec2;

use planet::*;
use vec2::*;

fn main() {
    // TODO: fix this shit
    // let v = Vec2::<i32>::zero();
    // let v2 = Vec2::<i32>::new(1, 1);
    // println!("{:?} {:?}", &v, &v2);

    // let v3 = v + v2;
    // println!("{:?} {:?} {:?}", &v, &v2, &v3);

    // let v4 = v + 10;
    // println!("{:?} {:?}", &v, &v4);

    let p = Planet::new(10.0, Vec2::zero(), Vec2::zero());
    let p2 = Planet::new(10.0, Vec2::new(0.0, 1.0), Vec2::zero());
    let mut planets = [p, p2];
    println!("{:?}", planets);

    for _ in 0..10 {
        sim::tick(0.1, 1.0, &mut planets);
        println!("{:#?}", planets);
    }
}
