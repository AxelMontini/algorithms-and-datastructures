/// Shitty recursive version of `F(i) = F(i - 1) + F(i - 2) + F(i - 3)`, `F(1) = F(2) = F(3) = 1`.
///
/// Top-down recursive
fn dp_recursive(num: usize) -> usize {
    if num == 0 {
        0
    } else if num <= 3 {
        1
    } else {
        dp_recursive(num - 1) + dp_recursive(num - 2) + dp_recursive(num - 3)
    }
}

/// Top-down memoized recursive solution
fn dp_memoized(num: usize) -> usize {
    if num == 0 {
        0
    } else if num <= 3 {
        1
    } else {
        let mut memo = vec![0; 1 + num as usize];

        memo[1] = 1;
        memo[2] = 1;
        memo[3] = 1;

        fn recursive(num: usize, memo: &mut [usize]) -> usize {
            let element = memo[num];
            if element == 0 {
                let val =
                    recursive(num - 1, memo) + recursive(num - 2, memo) + recursive(num - 3, memo);

                memo[num] = val;
            }

            memo[num]
        }

        recursive(num, &mut memo)
    }
}
fn dp_bottom_up(num: usize) -> usize {
    let mut table = Vec::with_capacity(num + 1);

    table.push(0);
    table.push(1);
    table.push(1);
    table.push(1);

    for i in 4..=num {
        table.push(table[i - 1] + table[i - 2] + table[i - 3]);
    }

    table[num]
}

#[cfg(test)]
mod tests {
    const DATASET: &[(usize, usize)] = &[(0, 0), (1, 1), (2, 1), (3, 1), (4, 3), (5, 5)];

    #[test]
    fn recursive() {
        for &(num, expected) in DATASET {
            assert_eq!(super::dp_recursive(num), expected);
        }
    }

    #[test]
    fn memoized() {
        for &(num, expected) in DATASET {
            assert_eq!(super::dp_memoized(num), expected);
        }
    }

    #[test]
    fn bottom_up() {
        for &(num, expected) in DATASET {
            assert_eq!(super::dp_bottom_up(num), expected);
        }
    }

    #[test]
    #[ignore]
    fn same_results() {
        for i in 0..50 {
            let memo = super::dp_memoized(i);
            let bu = super::dp_bottom_up(i);

            assert_eq!(memo, bu);

            if i < 40 {
                let rec = super::dp_recursive(i);
                assert_eq!(rec, memo);
            }
        }
    }
}
