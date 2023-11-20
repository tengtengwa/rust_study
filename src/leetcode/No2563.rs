fn main() {
    let ans = Solution::count_fair_pairs(vec![0, 1, 7, 4, 4, 5], 3, 6);
    println!();
    println!("{}", ans);
}

struct Solution {}

/**
* 2563. 统计公平数对的数目
   给你一个下标从 0 开始、长度为 n 的整数数组 nums ，和两个整数 lower 和 upper ，返回 公平数对的数目 。
   如果 (i, j) 数对满足以下情况，则认为它是一个 公平数对 ：
   0 <= i < j < n，且lower <= nums[i] + nums[j] <= upper
   例如：0, 1, 7, 4, 4, 5   公平数对的下标为(0,3)、(0,4)、(0,5)、(1,3)、(1,4) 和 (1,5)
*/
impl Solution {
    /**
     * 解法：双指针
     * 设sum为两数之和，要让两数在lower<= sum <=upper区间，可以将这个区间拆分为sum < lower和sum <= upper两部分
     * 再让后者的结果减去前者，实际上就是这个所要求的区间。
     * 时间复杂度：O(nlogn)
     */
    //变量是否为mut不影响其类型，排序需要可变类型，可以直接在nums前加mut
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort();
        let big = Solution::get_count(&nums, upper);
        let small = Solution::get_count(&nums, lower - 1);
        return big - small;
    }

    //我们先让l指向0，r指向len-1
    pub fn get_count(nums: &Vec<i32>, upper: i32) -> i64 {
        let mut ans = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            let sum = nums.get(l).unwrap() + nums.get(r).unwrap();
            //当sum<=upper时，表明这个区间[l,r]中，l和其他任一的数都满足条件，共r-l个，此时左指针右移
            if sum <= upper {
                ans += r - l;
                l += 1;
            } else {
                //和sum>upper，r指针指向的数过大，右指针左移
                r -= 1;
            }
        }
        return ans as i64;
    }
}
