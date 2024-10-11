/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    min: bool,
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T: Ord> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool, min: bool) -> Self {
        Self {
            min,
            count: 0,
            items: vec![],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.items.push(value);
        let mut index = self.items.len() - 1 ;
        match self.min {
            true => {
                println!("goes to minHeap");
                while index > 0 {
                    let parent_idx: usize = self.parent_idx(index);
                    
                    match (self.comparator)(&self.items[index], &self.items[parent_idx]) {
                        true => {
                            println!("minHeap index: {}  parent_idx: {}  ",
                        index,  parent_idx
                    );
                            self.items.swap(index, parent_idx);
                            index = parent_idx;
                        }
                        false => {
                            println!("doesn't exchange");
                            break;
                        }
                    }
                }
            }
            false => {
                while index > 0 {
                    let parent_idx: usize = self.parent_idx(index);
                    
                    match (self.comparator)(&self.items[index], &self.items[parent_idx]) {
                        true => {
                            println!("maxHeap index: {}  parent_idx: {}  ",
                                index,  parent_idx
                            );
                            self.items.swap(index, parent_idx);
                            index = parent_idx;
                        }
                        false => {
                            println!("doesn't exchange");
                            break;
                        }
                    }
                }
            }
        }

        self.count += 1;
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
		0
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b, true)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b, false)
    }
}

impl<T: Ord> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.items.len() == 0 {
            return None;
        }
		let item = self.items.remove(0);
        
        // shift down, swap last to top, and compare until satisfy the heap property
        let len = self.items.len() - 1;
        self.items.swap(0, len);
        let mut index = 0;
        
        match self.min {
            true => {
                println!("goes to minHeap next");
                loop {
                    let left_child = 2 * index +1;
                    let right_child = 2 * index + 2;
                    let len = self.items.len();
                    
                    let mut smallest = if left_child < len && self.items[left_child] < self.items[index] {
                        left_child
                    } else {
                        index
                    };
                    if right_child < len && self.items[right_child] < self.items[smallest] {
                        smallest = right_child;
                    }

                    if smallest != index {
                        self.items.swap(index, smallest);
                        index = smallest;
                    } else {
                        break;
                    }
                }
            }
            false => {
                println!("goes to maxHeap next");
                loop {
                    let left_child = 2 * index +1;
                    let right_child = 2 * index + 2;
                    let len = self.items.len();
                    
                    let mut biggest = if left_child < len && self.items[left_child] > self.items[index] {
                        left_child
                    } else {
                        index
                    };
                    if right_child < len && self.items[right_child] > self.items[biggest] {
                        biggest = right_child;
                    }

                    if biggest != index {
                        self.items.swap(index, biggest);
                        index = biggest;
                    } else {
                        break;
                    }
                }
            }
        }
        
        self.count -= 1;
        return Some(item);
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b, true)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b, false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}