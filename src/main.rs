fn main() {
    let ans = Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6);
    println!("{}", ans);
}

struct Solution {}

/**
* 2563. 统计公平数对的数目
   给你一个下标从 0 开始、长度为 n 的整数数组 nums ，和两个整数 lower 和 upper ，返回 公平数对的数目 。
   如果 (i, j) 数对满足以下情况，则认为它是一个 公平数对 ：
   0 <= i < j < n，且
   lower <= nums[i] + nums[j] <= upper
*/
impl Solution {
    pub fn count_fair_pairs(nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        let mut ans = 0;
        let mut arrs = nums;
        arrs.sort();
        let mut l = 0;
        let mut r = arrs.len();

        while (l < r) {
            let sum = arrs.get(l).unwrap_or(&0) + arrs.get(r).unwrap_or(&0);
            if lower <= sum && sum <= upper {
                ans += 1;
            } else if sum < lower {
                l += 1;
            } else if sum > upper {
                r -= 1;
            }
        }
        return ans;
    }
}
