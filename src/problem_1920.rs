pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
    nums.iter().map(|&i| nums[i as usize]).collect()
}

pub fn build_array_take_1(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::with_capacity(nums.len());
    for i in 0..nums.len() {
        let j = nums[i] as usize;
        ret[i] = nums[j];
    }
    ret
}

pub fn build_array_take_2(nums: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::with_capacity(nums.len());
    for i in &nums {
        ret.push(nums[*i as usize]);
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_array_works() {
        assert_eq!(build_array([0, 2, 1, 5, 3, 4].to_vec()), [0, 1, 2, 4, 5, 3]);
        assert_eq!(build_array([5, 0, 1, 2, 3, 4].to_vec()), [4, 5, 0, 1, 2, 3]);
    }
}
