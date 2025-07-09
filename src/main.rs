mod heap;
mod merge;

use crate::{
    heap::{heapsort_fast_construct, heapsort_slow_construct},
    merge::mergesort,
};
use rand::Rng;
use std::{
    rc::Rc,
    time::{self, Duration},
};
fn main() {
    let n = 10000;
    let loop_n = 100;
    let vec = gen_random_vec(n);

    track_avr(
        heapsort_slow_construct,
        &vec,
        "heapsort_slow_construct",
        loop_n,
    );
    // track_avr(
    //     heapsort_fast_construct,
    //     &vec,
    //     "heapsort_fast_construct",
    //     loop_n,
    // );
    // track_avr(mergesort, &vec, "mergesort", loop_n);
}

fn track_avr(
    to_track: impl Fn(&[i32]) -> Vec<i32>,
    data: &[i32],
    label: &str,
    loop_n: usize,
) -> Duration {
    let mut durations = vec![];
    for _ in 0..loop_n {
        let d = track(|| to_track(data), None);
        durations.push(d);
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
