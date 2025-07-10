use rand::Rng;

//mutate the arr instead returning new array
pub fn quicksort(arr : &mut [i32], rng : &mut rand::rngs::ThreadRng) {
    if arr.len() <= 1 {
        return;
    }
    let pivot = {
        let selected_pivot_idx = rng.random_range(0..arr.len()-1);
        arr.swap(selected_pivot_idx, arr.len()-1);
        arr[arr.len()-1]
    };
    let mut last_high_idx = 0;
    for i in 0 .. arr.len()-1 {
        if arr[i] < pivot {
            arr.swap(last_high_idx, i);
            last_high_idx += 1;
        }
    }
    arr.swap(last_high_idx, arr.len()-1);
    let (first_half, second_half) = arr.split_at_mut(last_high_idx);
    quicksort(first_half, rng);
    quicksort(&mut second_half[1..], rng);
} 