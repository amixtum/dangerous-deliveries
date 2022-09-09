pub struct Heap<K, T> 
where K: Clone,
      K: Copy,
      K: Ord,
      T: Clone, 
      T: Copy,
{
    heap: Vec<(K, T)>,
}

impl<K, T> Heap<K, T> 
where K: Clone,
      K: Copy,
      K: Ord,
      T: Clone, 
      T: Copy,
{
    pub fn new() -> Self {
        Heap {
            heap: Vec::new(),
        }
    }
}

impl<K, T> Heap<K, T> 
where K: Clone,
      K: Copy,
      K: Ord,
      T: Clone, 
      T: Copy,
{
    pub fn empty(&self) -> bool {
        return self.heap.len() < 2;
    }

    pub fn insert(&mut self, key: K, val: T) {
        if self.heap.len() < 2 {
            if self.heap.len() == 1 {
                self.heap.remove(0);
            }
            self.heap.push((key, val));
            self.heap.push((key, val));
        }
        else {
            self.heap.push((key, val));
            let mut index = self.heap.len() - 1;
            while self.heap[parent(index)].0 > key && index > 1 {
                self.swap(parent(index), index);
                index = parent(index);
            }
        }
    }

    pub fn extract_min(&mut self) -> T {
        let l  = 1;
        let r = self.heap.len() - 1;
        self.swap(l, r);

        let min = self.heap.remove(r);

        let mut index = 1;
        while index < self.heap.len() / 2 {
            if self.heap[index].0 <= self.heap[left(index)].0 ||
               self.heap[index].0 <= self.heap[right(index)].0 {
                break;
            }
            
            let l = index;
            let r: usize;
            if right(index) >= self.heap.len() {
                r = left(index);
            }
            else if self.heap[left(index)].0 < self.heap[right(index)].0 {
                r = left(index);
            }
            else {
                r = right(index);
            }

            self.swap(l, r);
            index = r;
        }

        min.1
    }

    fn swap(&mut self, l: usize, r: usize) {
        let temp = self.heap[l];
        self.heap[l] = self.heap[r];
        self.heap[r] = temp;
    }
}

fn parent(index: usize) -> usize {
    index / 2
}

fn left(index: usize) -> usize {
    index * 2
}

fn right(index: usize) -> usize {
    index * 2 + 1
}