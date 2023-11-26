fn main() {
    for i in 1..=5 {
        let ans = Solution::count_and_say(i);
        println!("{}", ans);
    }
}

struct Solution {}

/**
 * 38. 外观数列
 * https://leetcode.cn/problems/count-and-say/description/
 */
impl Solution {
    /**
     * 解法一：遍历
     * 按照题目所述，在每次遍历中生成新的字符串即可
     * 时间复杂度：O(N*M)，N为传入的N，M为生成的字符串中最长的长度
     */
    pub fn count_and_say(n: i32) -> String {
        let mut str = String::from("1");
        for i in 2..n + 1 {
            let mut tem = String::new();
            let mut start = 0;
            let mut pos = 0;
            while pos < str.len() {
                //获取字符串中的字符切片，通过索引运算符'[]'
                //str[s..e]取的范围是开区间[s,e)，通过'..='操作符可以取闭区间[s,e]
                while pos < str.len() && str[start..=start] == str[pos..=pos] {
                    pos += 1;
                }
                tem.push_str(&(pos - start).to_string());
                tem.push_str(&str[start..=start]);
                start = pos;
            }
            str = tem;
        }

        return str;
    }
}
