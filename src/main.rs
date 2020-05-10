mod util;
mod vec2;

use vec2::*;

fn main() {
    let v = Vec2::<i32>::zero();
    let v2 = Vec2::<i32>::new(1, 1);
    println!("{:?} {:?}", &v, &v2);

    let v3 = v + v2;
    println!("{:?} {:?} {:?}", &v, &v2, &v3);

    let v4 = v + 10;
    println!("{:?} {:?}", &v, &v4);
}
