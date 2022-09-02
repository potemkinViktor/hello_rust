#[derive(Debug)]
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use TrafficLight::{Red, Yellow};

#[derive(Debug)]
enum Traffic {
    R,
    Y,
    G,
}

use Traffic::*;

pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_modules() {
                println!("nested_modules");
            }
        }
    }
}

use a::series::of;

use a::series::of::nested_modules;

fn main() {
    of::nested_modules();
    nested_modules();

    let red = Red;
    let yellow = Yellow;
    let green = TrafficLight::Green;

    println!("{:?}", red);
    println!("{:?}", yellow);
    println!("{:?}", green);

    let r = R;
    let y = Y;
    let g = G;

    println!("{:?}", r);
    println!("{:?}", y);
    println!("{:?}", g);
}
