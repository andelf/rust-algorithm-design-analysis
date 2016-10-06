use std::ops::Deref;
use std::cmp;
use std::collections::BinaryHeap;

/// A wrapper around Ord values, inverse its order.
#[derive(PartialEq, Eq)]
pub struct ReverseOrder<T>(T);

impl<T: Ord> ReverseOrder<T> {
    pub fn new(val: T) -> ReverseOrder<T> {
        ReverseOrder(val)
    }

    pub fn unwrap(self) -> T {
        self.0
    }

    pub fn as_ref(&self) -> &T {
        &self.0
    }
}

impl<T: PartialOrd> PartialOrd for ReverseOrder<T> {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.0.partial_cmp(&other.0).map(|ord| ord.reverse())
    }
}


impl<T: Ord> Ord for ReverseOrder<T> {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl<T: Ord + Default> Default for ReverseOrder<T> {
    fn default() -> Self {
        ReverseOrder::new(Default::default())
    }
}

impl<T: Ord> Deref for ReverseOrder<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

#[test]
fn test_reverse_order() {
    assert!(ReverseOrder::new(1) == ReverseOrder::new(1));
    assert!(ReverseOrder::new(1) >= ReverseOrder::new(2));
}



pub struct MedianMaintainer<T> {
    h_low: BinaryHeap<T>,
    h_high: BinaryHeap<ReverseOrder<T>>,
}


impl<T: Ord> MedianMaintainer<T> {
    pub fn new() -> MedianMaintainer<T> {
        MedianMaintainer {
            h_low: Default::default(), // MaxHeap
            h_high: Default::default(), // MinHeap
        }
    }

    pub fn push(&mut self, val: T) {
        if self.h_low.peek().map_or(false, |l| &val <= l) {
            self.h_low.push(val);
        } else {
            // for val > h_high.min, or other case
            self.h_high.push(ReverseOrder::new(val));
        }

        if self.h_high.len() == self.h_low.len() + 2 {
            self.h_high.pop()
                .map(|v| self.h_low.push(v.unwrap())); // unwrap reverse order wrapper
        } else if self.h_low.len() > self.h_high.len() {
            self.h_low.pop()
                .map(|v| self.h_high.push(ReverseOrder::new(v)));
        }
    }

    pub fn median(&self) -> Option<&T> {
        if self.h_high.len() == self.h_low.len() { // even item,
            self.h_low.peek()
        } else {                // odd item
            self.h_high.peek().map(|v| v.as_ref())
        }
    }
}


#[test]
fn test_median_maintainer() {
    let mut mm = MedianMaintainer::new();

    let medians = vec![7, 2, 4, 4, 4, 3, 4];
    for (i, val) in vec![7, 2, 4, 7, 1, 3, 9].into_iter().enumerate() {
        mm.push(val);
        assert_eq!(mm.median().unwrap_or(&0), &medians[i], "medians verification");
    }
}
