// Copyright (c) 2016 project developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.


pub fn quick_sort(arr: &mut [i32]) -> usize {
    if arr.len() <= 1 {
        return 0;
    }
    let pivot = choose_pivot(arr);
    if pivot != 0 {
        arr.swap(0, pivot);
    }
    let mid = inplace_partition(arr);
    let x = quick_sort(&mut arr[..mid-1]);
    let y = quick_sort(&mut arr[mid..]);
    // when there is a recursive call on a subarray of length m,
    // you should simply add m−1 to your running total of comparisons.
    // (This is because the pivot element is compared to each of
    // the other m−1 elements in the subarray in this recursive call.)
    arr.len() - 1 + x + y
}



fn choose_pivot(arr: &[i32]) -> usize {
    // 0
    //arr.len() - 1
    median_of_three(arr)
}


fn median_of_three(arr: &[i32]) -> usize {
    let first = 0;
    // IMPORTANT: middle of array
    let middle = (arr.len() + 1) / 2 - 1;
    let last = arr.len() - 1;
    let a = arr[first];
    let b = arr[middle];
    let c = arr[last];

    if (a-b)*(b-c) >= 0 {
        middle
    } else if (a-b)*(a-c) <= 0 {
        first
    } else {
       last
    }
}

// assume pivot is first
fn inplace_partition(arr: &mut [i32]) -> usize {
    let pivot = 0;
    let n = arr.len();
    let p = arr[pivot];
    let mut i = pivot + 1;

    for j in pivot+1 .. n {
        if arr[j] < p {
            arr.swap(j, i);
            i += 1
        }
    }
    arr.swap(pivot, i-1);
    i
}


#[test]
fn test_inplace_partion() {
    let mut v = vec![3, 8, 2, 5, 1, 4, 7, 6];
    inplace_partition(&mut v[..]);
    assert_eq!(&v, &[1, 2, 3, 5, 8, 4, 7, 6]);
}


#[test]
fn test_quick_sort() {
    let mut v = vec![3, 8, 2, 5, 1, 4, 7, 6];
    let cmp_cnt = quick_sort(&mut v[..]);
    assert_eq!(&v, &[1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(cmp_cnt, 13);
}

#[test]
fn test_mid_of_three() {
    assert_eq!(median_of_three(&[8, 2, 4, 5, 7, 1]), 2);
    assert_eq!(median_of_three(&[1, 3, 8]), 1);
}
