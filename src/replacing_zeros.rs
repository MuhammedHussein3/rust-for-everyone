impl Solution {
    pub fn min_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        fn compute_sum_and_zeros(nums: &[i32]) -> (i64, i32) {
            nums.iter().fold((0, 0), |(sum, zeros), &num| {
                if num == 0 {
                    (sum + 1, zeros + 1)
                }else {
                    (sum + num as i64, zeros)
                }
            })
        }

        let (sum1, zeros1) = compute_sum_and_zeros(&nums1);
        let (sum2, zeros2) = compute_sum_and_zeros(&nums2);

        if sum1 == sum2 {
            return sum1;
        } else if sum1 > sum2 && zeros2 > 0 {
            return sum1;
        }else if sum1 < sum2 && zeros1 > 0 {
            return sum2;
        } else {
            -1
        }
    }
    
}