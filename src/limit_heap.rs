use std::{cmp::Reverse, collections::BinaryHeap, iter::Map};

pub struct LimitHeap<T: Ord, const N: usize>(BinaryHeap<Reverse<T>>);

impl<T: Ord, const N: usize> LimitHeap<T, N> {
    pub fn new() -> Self {
        Self(BinaryHeap::new())
    }

    pub fn push(&mut self, item: T) {
        if self.0.len() < N {
            self.0.push(Reverse(item));
            return;
        }

        match self.0.peek() {
            Some(Reverse(smallest_value)) if smallest_value < &item => {
                self.0.pop();
                self.0.push(Reverse(item))
            }
            _ => {}
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|reversed_item| &reversed_item.0)
    }
}

impl<A: Ord, const N: usize> FromIterator<A> for LimitHeap<A, N> {
    fn from_iter<T: IntoIterator<Item = A>>(iter: T) -> Self {
        let mut heap = Self::new();

        for item in iter {
            heap.push(item)
        }

        heap
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn limits_amount() {
        let mut heap = super::LimitHeap::<u32, 3>::new();

        heap.push(2);
        heap.push(3);
        heap.push(4);
        heap.push(5);
        heap.push(1);

        assert_eq!(heap.iter().copied().collect::<Vec<_>>(), vec![3, 4, 5])
    }
}
