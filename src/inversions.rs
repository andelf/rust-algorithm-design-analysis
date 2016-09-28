pub fn merge_sort_and_count_inversions(seq: &mut [usize]) -> usize {
    if seq.len() <= 1 {
        0
    } else {
        let mid = seq.len() / 2;
        let x = merge_sort_and_count_inversions(&mut seq[..mid]);
        let y = merge_sort_and_count_inversions(&mut seq[mid..]);
        let z = merge(&mut seq[..], mid);

        x + y + z
    }
}

// In merge process, let i is used for indexing left sub-array and j for right sub-array.
// At any step in merge(), if a[i] is greater than a[j], then there are (mid – i) inversions.
// because left and right subarrays are sorted, so all the remaining elements
// in left-subarray (a[i+1], a[i+2] … a[mid]) will be greater than a[j];
// [*]: http://www.geeksforgeeks.org/counting-inversions/
fn merge(seq: &mut [usize], mid: usize) -> usize {
    let mut inv_cnt = 0;
    let mut i = 0;
    let mut j = mid;
    let n = seq.len();

    let mut temp = Vec::with_capacity(n);

    while i <= mid - 1 && j < n {
        if seq[i] <= seq[j] {
            temp.push(seq[i]);
            i += 1;
        } else {
            inv_cnt += mid - i;
            temp.push(seq[j]);
            j += 1;
        }
    }

    while i <= mid - 1 {
        temp.push(seq[i]);
        i += 1;
    }

    while j < n {
        temp.push(seq[j]);
        j += 1;
    }

    seq.clone_from_slice(&temp);
    inv_cnt
}


#[cfg(test)]
mod tests {
    use self::super::*;

    #[test]
    fn test_num_of_inversions() {
        assert_eq!(merge_sort_and_count_inversions(&mut [1, 3, 5, 2, 4, 6]), 3);
        assert_eq!(merge_sort_and_count_inversions(&mut [1, 5, 3, 2, 4]), 4);
        assert_eq!(merge_sort_and_count_inversions(&mut [5, 4, 3, 2, 1]), 10);
        assert_eq!(merge_sort_and_count_inversions(&mut [1, 6, 3, 2, 4, 5]), 5);
    }

}
