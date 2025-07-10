use std::mem;

use rand::Rng;

//mutate the arr instead returning new array
pub fn quicksort(arr : &mut [i32], rng : &mut rand::rngs::ThreadRng) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = {
        let selected_pivot_idx = rng.random_range(0..arr.len()-1);
        if selected_pivot_idx != arr.len()-1{
            let (arr, last) = arr.split_at_mut(arr.len()-1);
            mem::swap(&mut arr[selected_pivot_idx],&mut last[0]);
        }
        arr[arr.len()-1]
    };
    let mut last_high_idx = 0;
    for i in 0 .. arr.len()-1 {
        if arr[i] < pivot {
            if last_high_idx != i {
                let (first_half, lower_than_pivot) = arr.split_at_mut(i);
                mem::swap(&mut first_half[last_high_idx],&mut lower_than_pivot[0]);
            };
            last_high_idx += 1;
        }
    }
    if last_high_idx != arr.len()-1 {
        let (first_half, pivot) = arr.split_at_mut(arr.len()-1);
        mem::swap(&mut first_half[last_high_idx],&mut pivot[0]);
    }

    let (first_half, second_half) = arr.split_at_mut(last_high_idx);
    quicksort(first_half, rng);
    quicksort(&mut second_half[1..], rng);
} 