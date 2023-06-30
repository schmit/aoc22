use std::collections::BinaryHeap;
use std::cmp::{Ordering, Reverse};

pub struct TopK<T> {
    heap: BinaryHeap<Reverse<T>>,
    k: usize,
}

impl <T> TopK<T> 
where
    T: Ord + Copy
{
    pub fn new(k: usize) -> Self {
        TopK {
            // This is a min-heap, so when we pop the "largest" value from the heap
            // this is actually the k-th value in the list of largest values.
            heap: BinaryHeap::with_capacity(k),
            k: k,
        }
    }

    pub fn push(&mut self, item: T) {
        if self.heap.len() < self.k {
            self.heap.push(Reverse(item));
        } else {
            // Compare item to the smallest value in the heap
            // if it is larger, then pop smallest value and add current item
            if let Some(smallest) = self.peek() {
                match item.cmp(smallest) {
                    Ordering::Greater => {
                        self.heap.pop();
                        self.heap.push(Reverse(item));
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn peek(&mut self) -> Option<&T> {
        self.heap
            .peek()
            .map(|Reverse(item)| item)
    }

    pub fn values(self) -> Vec<T> {
        self.heap
            .into_sorted_vec()
            .into_iter()
            .map(|Reverse(item)| item)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topk() {
        let mut topk = TopK::new(3);
        topk.push(1);
        topk.push(2);
        topk.push(3);
        topk.push(4);
        topk.push(5);
        assert_eq!(topk.values(), vec![5, 4, 3]);
    }

    #[test]
    fn test_topk_not_full() {
        let mut topk = TopK::new(3);
        topk.push(1);
        assert_eq!(topk.values(), vec![1]);
    }

    #[test]
    fn test_topk_empty() {
        let topk = TopK::<i32>::new(3);
        assert_eq!(topk.values(), vec![]);
    }
}