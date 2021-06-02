pub fn shuffle(nums: Vec<i32>, n: i32) -> Vec<i32> {
    let n = n as usize;
    let mut result = Vec::with_capacity(n);
    for i in 0..n {
        result.push(nums[i]);
        result.push(nums[n + i]);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shuffle_works() {
        let input = vec![2, 5, 1, 3, 4, 7];
        let expected = vec![2, 3, 5, 4, 1, 7];
        let result = shuffle(input, 3);
        assert_eq!(expected, result);
    }
}
