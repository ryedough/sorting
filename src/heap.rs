/// Construct heap by inserting each element of the unsorted array into the heap
pub fn heapsort_slow_construct(arr: &[i32]) -> Vec<i32> {
    let arr = Vec::from(arr);
    let mut heap = MinHeap::with_capacity(arr.len());
    for v in arr {
        heap.insert(v);
    }
    let mut sorted = vec![];
    while heap.len() > 0 {
        sorted.push(heap.extract());
    }
    sorted
}

/// Consturct heap by borrowing the unsorted array, and using bubble down from (n/2)th node until 0th node to create the heap.
/// 
/// This is faster than `heapsort_slow_construct` because it does not require inserting
/// # Return
/// The sorted array
/// # Note
/// Will mutate the original array into gibberish
pub fn heapsort_fast_construct(arr: &mut [i32]) -> Vec<i32> {
    let mut sorted = Vec::with_capacity(arr.len());
    let mut heap = MinHeapBorrow::from(arr);
    while heap.len() > 0 {
        sorted.push(heap.extract());
    }
    sorted
}

struct MinHeap {
    data: Vec<i32>,
    n: usize,
}

impl MinHeap {
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn with_capacity(n: usize) -> MinHeap {
        MinHeap {
            data: Vec::with_capacity(n),
            n: 0,
        }
    }
    pub fn insert(&mut self, v: i32) {
        self.data.push(v);
        self.n += 1;
        if self.n == 0 {
            return;
        } else {
            self.bubble_up(self.n - 1);
        }
    }
    pub fn extract(&mut self) -> i32 {
        let result = self.data[0];
        self.data[0] = self.data[self.n - 1];
        self.n -= 1;

        if self.n > 0 {
            self.bubble_down(0);
        }

        result
    }
    fn bubble_up(&mut self, node_idx: usize) {
        let parent_idx = match self.parent(node_idx) {
            Some(idx) => idx,
            None => return,
        };

        if self.data[node_idx] < self.data[parent_idx] {
            self.data.swap(parent_idx, node_idx);
        }
        self.bubble_up(parent_idx);
    }

    fn bubble_down(&mut self, node_idx: usize) {
        let child_idx = self.child(node_idx);
        let mut smallest_idx = node_idx;

        for opt in [child_idx.0, child_idx.1] {
            if let Some(idx) = opt {
                if self.data[smallest_idx] > self.data[idx] {
                    smallest_idx = idx;
                }
            }
        }

        if smallest_idx == node_idx {
            return;
        }
        self.data.swap(node_idx, smallest_idx);
        self.bubble_down(smallest_idx);
    }

    fn parent(&self, mut node_idx: usize) -> Option<usize> {
        assert!(node_idx <= self.n);

        node_idx += 1; //add 1 to make idx starts from 1 (ensuring the math works)
        if node_idx <= 1 {
            return None;
        }

        // decreasing parent idx by one so it work with normal arr
        Some((node_idx / 2) - 1)
    }

    fn child(&self, node_idx: usize) -> (Option<usize>, Option<usize>) {
        //add 1 to make idx starts from 1 (ensuring the math works)
        let l_child_idx = (node_idx + 1) * 2;
        if l_child_idx > self.n {
            return (None, None);
        }
        let r_child_idx = match l_child_idx.cmp(&(self.n - 1)) {
            std::cmp::Ordering::Greater => None,
            _ => Some(l_child_idx), //using left child index because left child will be decreased by one
        };

        //decreasing left child by one
        (Some(l_child_idx - 1), r_child_idx)
    }
}

struct MinHeapBorrow<'a> {
    data: &'a mut [i32],
    n: usize,
}

impl<'a> MinHeapBorrow<'a> {
    pub fn len(&self) -> usize {
        self.n
    }
    pub fn from(vec: &'a mut [i32]) -> MinHeapBorrow<'a> {
        let len = vec.len();
        let mut heap = MinHeapBorrow { data: vec, n: len };
        for i in (0..(len / 2) - 1).rev() {
            heap.bubble_down(i);
        }
        heap
    }
    pub fn extract(&mut self) -> i32 {
        let result = self.data[0];
        self.data.swap(0, self.n-1);
        self.n -= 1;

        if self.n > 0 {
            self.bubble_down(0);
        }

        result
    }
    fn bubble_down(&mut self, node_idx: usize) {
        let child_idx = self.child(node_idx);
        let mut smallest_idx = node_idx;

        for opt in [child_idx.0, child_idx.1] {
            if let Some(idx) = opt {
                if self.data[smallest_idx] > self.data[idx] {
                    smallest_idx = idx;
                }
            }
        }
        if smallest_idx == node_idx {
            return;
        }
        self.data.swap(node_idx, smallest_idx);
        self.bubble_down(smallest_idx);
    }
    fn child(&self, node_idx: usize) -> (Option<usize>, Option<usize>) {
        //add 1 to make idx starts from 1 (ensuring the math works)
        let l_child_idx = (node_idx + 1) * 2;
        if l_child_idx > self.n {
            return (None, None);
        }
        let r_child_idx = match l_child_idx.cmp(&(self.n - 1)) {
            std::cmp::Ordering::Greater => None,
            _ => Some(l_child_idx), //using left child index because left child will be decreased by one
        };

        //decreasing left child by one
        (Some(l_child_idx - 1), r_child_idx)
    }
}
