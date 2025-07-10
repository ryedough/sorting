mod heap;
mod merge;
mod quick;

use crate::{
    heap::{heapsort_fast_construct, heapsort_slow_construct},
    merge::mergesort, quick::quicksort,
};
use rand::Rng;
use std::{
    rc::Rc,
    time::{self, Duration},
};
fn main() {
    let n = 1_000_000;
    let loop_n = 10;
    let vec = gen_random_vec(n);

    // track_avr(|| {
    //     let mut vec = vec.clone();
    //     track(move||{
    //         heapsort_slow_construct(&mut vec);
    //     }, None)
    // }, "heapsort_slow_construct", loop_n);

    track_avr(|| {
        let mut vec = vec.clone();
        track(move||{
            heapsort_fast_construct(&mut vec);
            // println!("{res:?}")
        }, None)
    }, "heapsort_fast_construct", loop_n);

    track_avr(|| {
        let mut vec = vec.clone();
        track(move||{
            mergesort(&mut vec);
            // println!("{res:?}")
        }, None)
    }, "mergesort", loop_n);

    // track_avr(|| {
    //     let mut vec = vec.clone();
    //     let mut rng = rand::rng();
    //     track(move||{
    //         quicksort(&mut vec, &mut rng);
    //     }, None)
    // }, "quicksort", loop_n);
}

fn track_avr(
    to_track: impl Fn() -> Duration,
    label: &str,
    loop_n: usize,
) -> Duration {
    let mut durations = Vec::with_capacity(loop_n);
    for _ in 0..loop_n {
        durations.push(to_track());
    }
    let sum = durations.into_iter().reduce(|acc, v| acc + v).unwrap();
    let avg = sum / loop_n.try_into().unwrap();
    println!("{label} -- Average of {loop_n} loop(s), took : {avg:?}");
    avg
}

fn track<T>(to_track: impl FnOnce() -> T, label: Option<&str>) -> Duration {
    let start = time::SystemTime::now();
    to_track();
    let duration = time::SystemTime::now().duration_since(start).unwrap();
    if let Some(label) = label {
        println!("{label} -- Time took : {:?}", duration);
    }
    duration
}

fn gen_random_vec(n: usize) -> Vec<i32> {
    let mut rng = rand::rng();
    let mut vec: Vec<i32> = Vec::with_capacity(n);
    for _ in 0..n {
        vec.push(rng.random_range(0..n.try_into().unwrap()));
    }
    // println!("generated vec with n = {n} : {vec:?}");
    vec
}
