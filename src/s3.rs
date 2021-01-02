/// # Task
/// Compute the maximum contiguous subarray difference in O(n)
///
/// # Solution
/// Using linear space it's possible to find the max subarray up until index `b` and the min
/// subarray starting at index `b` (in O(n)). This way, the next step is computationally easy.
/// Then the max difference is computed in O(n).
fn maximum_subarray_difference(arr: &[i32]) -> i32 {
    if arr.len() == 0 {
        return 0;
    }

    let mut max = vec![0; arr.len() + 1];
    let mut min = vec![0; arr.len() + 1];
    let mut max_diff = 0;

    // Fill max and min
    for idx in 1..arr.len() {
        max[idx + 1] = arr[idx - 1].max(max[idx] + arr[idx - 1]);
        min[arr.len() - idx] =
            arr[arr.len() - idx].min(min[arr.len() - idx + 1] + arr[arr.len() - idx]);
    }

    for (curr_max, curr_min) in max.iter().skip(1).zip(min.iter()) {
        if max_diff < curr_max - curr_min {
            max_diff = curr_max - curr_min;
        }
    }

    max_diff
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_subarray_diff() {
        let dataset = &[(&[1, 2, 3, 4, 5], 5), (&[-1, -5, -5, -5, -1], 15)];

        for &(arr, expected) in dataset {
            assert_eq!(maximum_subarray_difference(arr), expected);
        }
    }
}
