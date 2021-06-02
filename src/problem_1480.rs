pub fn running_sum_iter(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter()
        .map(|num| {
            let next = sum + num;
            sum += num;
            next
        })
        .collect()
}

pub fn running_sum_for(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut result = Vec::with_capacity(n);
    if n > 0 {
        result.push(nums[0]);
        for i in 1..n {
            result.push(result[i - 1] + nums[i]);
        }
        result
    } else {
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn running_sum_iter_works() {
        let input = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        let result = running_sum_iter(input);
        assert_eq!(expected, result);
    }

    #[test]
    fn running_sum_for_works() {
        let input = vec![3, 1, 2, 10, 1];
        let expected = vec![3, 4, 6, 16, 17];
        let result = running_sum_for(input);
        assert_eq!(expected, result);
    }
}
