extern crate rand;
use rand::Rng;

pub struct Wheel {
  
}

impl Wheel {
  pub fn new() -> Wheel {
    Wheel {

    }
  }
}

fn spin_wheel() -> i32 {
    let mut rng = rand::thread_rng();
    let wheel = vec![2500, 600, 700, 600, 650, 500, 700, 600, 550, 500, 600, 650, 700, 800, 500, 650, 500, 900];
    let index = rng.gen_range(0, wheel.len());

    wheel[index]
}