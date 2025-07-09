#[allow(dead_code)]
pub fn mergesort(arr: &[i32]) -> Vec<i32> {
    if arr.len() == 1 {
        return Vec::from(arr);
    };
    let mid = arr.len() / 2;
    let (arr_a, arr_b) = arr.split_at(mid);
    let arr_a = mergesort(arr_a);
    let arr_b = mergesort(arr_b);

    let mut result = vec![];
    {
        let mut a_idx = 0;
        let mut b_idx = 0;
        while a_idx < arr_a.len() || b_idx < arr_b.len() {
            let a = match a_idx.cmp(&arr_a.len()) {
                std::cmp::Ordering::Less => Some(arr_a[a_idx]),
                _ => None,
            };
            let b = match b_idx.cmp(&arr_b.len()) {
                std::cmp::Ordering::Less => Some(arr_b[b_idx]),
                _ => None,
            };
            if a.is_none() || b.is_none() {
                if let Some(a) = a {
                    result.push(a);
                    a_idx += 1;
                }
                if let Some(b) = b {
                    result.push(b);
                    b_idx += 1;
                }
                continue;
            }
            let (a, b) = (a.unwrap(), b.unwrap());

            match a.cmp(&b) {
                std::cmp::Ordering::Less => {
                    result.push(a);
                    a_idx += 1;
                }
                _ => {
                    result.push(b);
                    b_idx += 1;
                }
            };
        }
    };
    result
}
