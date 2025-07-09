mod heap;

use std::time::{self, Duration};
use rand::Rng;
use crate::heap::{heapsort_fast_construct, heapsort_slow_construct};
fn main() {
    let vec = gen_random_vec(1000000); 
    let vec2 = vec.clone();
    let a = track(move || {
        let res = heapsort_slow_construct(vec);
        // println!("{res:?}");
    }, "heapsort_slow_construct");

    let b = track(move || {
        let res = heapsort_fast_construct(vec2);
        // println!("{res:?}");
    }, "heapsort_fast_construct");
}

fn track(to_track : impl FnOnce(), label : &str) -> Duration {
    let start= time::SystemTime::now();
    to_track();
    let duration = time::SystemTime::now().duration_since(start).unwrap();
    println!("{label} -- Time took : {:?}", duration);
    duration
}

fn gen_random_vec(n : usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = Vec::with_capacity(n);
    for _ in 0 .. n {
        vec.push(rng.random_range(0..n.try_into().unwrap()));
    }
    // println!("generated vec with n = {n} : {vec:?}");
    vec
}
