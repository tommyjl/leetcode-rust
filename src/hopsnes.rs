//! Merge two sorted arrays
//!
//! Target array: [3, 5, 8, 17, 20, 22, 0, 0, 0, 0, 0]
//! Merger array: [2, 5, 9, 12, 15]
//!
//! The target array always contains the length of the merger array
//! initialized to 0 at the end of its sorted segment.

use std::mem::swap;

pub fn mergerino_slow(l: &mut [i32], r: &mut [i32]) {
    let mut r_idx = 0;
    for l_next in l.iter_mut() {
        if r_idx == r.len() {
            return;
        };

        if *l_next == 0 {
            swap(l_next, &mut r[r_idx]);
            r_idx += 1;
            continue;
        }

        if *l_next <= r[r_idx] {
            continue;
        }

        swap(l_next, &mut r[r_idx]);

        // One pass of bubble-sort
        let mut r_idx_next = r_idx;
        while r_idx_next < (r.len() - 1) {
            if r[r_idx_next] > r[r_idx_next + 1] {
                r.swap(r_idx_next, r_idx_next + 1);
                r_idx_next += 1;
            } else {
                break;
            }
        }
    }
}

pub fn mergerino_fast(target: &mut [i32], to_merge: &mut [i32]) {
    if target.len() == to_merge.len() {
        #[allow(clippy::manual_memcpy)]
        for i in 0..target.len() {
            target[i] = to_merge[i];
        }
        return;
    }

    let mut i = target.len() - to_merge.len() - 1;
    let mut j = to_merge.len() - 1;

    let mut i_done = false;
    let mut j_done = false;

    for end in (0..target.len()).rev() {
        if i_done || j_done {
            if target[i] == 0 {
                swap(&mut target[i], &mut to_merge[j]);
            }
            continue;
        }

        if target[i] > to_merge[j] {
            target.swap(end, i);
            if i > 0 {
                i -= 1;
            } else {
                i_done = true;
            }
        } else {
            swap(&mut target[end], &mut to_merge[j]);
            if j > 0 {
                j -= 1;
            } else {
                j_done = true;
            }
        }
    }
}

pub fn mergerino_fast_v2_assign(l: &mut [i32], r: &mut [i32]) {
    let (mut l_end, mut r_end) = {
        if l.is_empty() {
            (-1, -1)
        } else if l.len() == r.len() {
            (-1, r.len() as isize - 1)
        } else {
            (
                l.len() as isize - r.len() as isize - 1,
                r.len() as isize - 1,
            )
        }
    };

    for end in (0..l.len()).rev() {
        if r_end < 0 {
            return;
        }
        if l_end < 0 {
            l[end] = r[r_end as usize];
            r_end -= 1;
        } else if l[l_end as usize] > r[r_end as usize] {
            l[end] = l[l_end as usize];
            l_end -= 1;
        } else {
            l[end] = r[r_end as usize];
            r_end -= 1;
        };
    }
}

pub fn mergerino_fast_v2_swap(l: &mut [i32], r: &mut [i32]) {
    let (mut l_end, mut r_end) = {
        if l.is_empty() {
            (-1, -1)
        } else if l.len() == r.len() {
            (-1, r.len() as isize - 1)
        } else {
            (
                l.len() as isize - r.len() as isize - 1,
                r.len() as isize - 1,
            )
        }
    };

    for end in (0..l.len()).rev() {
        if r_end < 0 {
            return;
        }
        if l_end < 0 {
            swap(&mut l[end], &mut r[r_end as usize]);
            r_end -= 1;
        } else if l[l_end as usize] > r[r_end as usize] {
            l.swap(end, l_end as usize);
            l_end -= 1;
        } else {
            swap(&mut l[end], &mut r[r_end as usize]);
            r_end -= 1;
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mergerino_slow_1() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_slow(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_slow_2() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_slow(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_slow_3() {
        let mut target = vec![5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![5, 5, 5, 5, 5, 5];
        mergerino_slow(&mut target, &mut merger);
        assert_eq!(target, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn mergerino_slow_4() {
        let mut target = vec![0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_slow(&mut target, &mut merger);
        assert_eq!(target, vec![2, 5, 9, 17, 19, 22]);
    }

    #[test]
    fn mergerino_slow_5() {
        let mut target = vec![2, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![3, 5, 9, 17, 19, 22];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_1() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_2() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_3() {
        let mut target = vec![5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![5, 5, 5, 5, 5, 5];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn mergerino_fast_4() {
        let mut target = vec![0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![2, 5, 9, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_5() {
        let mut target = vec![2, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![3, 5, 9, 17, 19, 22];
        mergerino_fast(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_assign_1() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_assign(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_assign_2() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_assign(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_assign_3() {
        let mut target = vec![5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![5, 5, 5, 5, 5, 5];
        mergerino_fast_v2_assign(&mut target, &mut merger);
        assert_eq!(target, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn mergerino_fast_v2_assign_4() {
        let mut target = vec![0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_assign(&mut target, &mut merger);
        assert_eq!(target, vec![2, 5, 9, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_assign_5() {
        let mut target = vec![2, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![3, 5, 9, 17, 19, 22];
        mergerino_fast_v2_assign(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_swap_1() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_swap(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_swap_2() {
        let mut target = vec![3, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_swap(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_swap_3() {
        let mut target = vec![5, 5, 5, 5, 5, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![5, 5, 5, 5, 5, 5];
        mergerino_fast_v2_swap(&mut target, &mut merger);
        assert_eq!(target, vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5]);
    }

    #[test]
    fn mergerino_fast_v2_swap_4() {
        let mut target = vec![0, 0, 0, 0, 0, 0];
        let mut merger = vec![2, 5, 9, 17, 19, 22];
        mergerino_fast_v2_swap(&mut target, &mut merger);
        assert_eq!(target, vec![2, 5, 9, 17, 19, 22]);
    }

    #[test]
    fn mergerino_fast_v2_swap_5() {
        let mut target = vec![2, 5, 8, 10, 12, 0, 0, 0, 0, 0, 0];
        let mut merger = vec![3, 5, 9, 17, 19, 22];
        mergerino_fast_v2_swap(&mut target, &mut merger);
        assert_eq!(target, vec![2, 3, 5, 5, 8, 9, 10, 12, 17, 19, 22]);
    }
}
