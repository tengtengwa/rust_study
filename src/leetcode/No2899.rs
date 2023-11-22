fn main() {
    let ans = Solution::last_visited_integers(vec![
        "prev".to_string(),
        "prev".to_string(),
        "prev".to_string(),
        "52".to_string(),
        "prev".to_string(),
    ]);
    for n in ans {
        println!("{}", n);
    }
}

struct Solution {}

/**
 * 2899. 上一个遍历的整数
 * https://leetcode.cn/problems/last-visited-integers/description/
 */
impl Solution {
    pub fn last_visited_integers(words: Vec<String>) -> Vec<i32> {
        let mut ans = vec![];
        let mut nums = vec![];
        let mut nums_rvs = vec![];
        let mut k = 0; //连续的"prev"字符串数量
        for word in words {
            if Solution::is_number(&word) {
                k = 0;
                let n = word.parse::<i32>();
                if let Ok(a) = n {
                    nums.push(a);
                    nums_rvs.insert(0, a);
                }
            } else {
                k += 1;
                let mut a = -1;
                if let Some(i) = nums_rvs.get(k - 1) {
                    a = *i;
                }
                ans.push(a);
            }
        }
        return ans;
    }

    fn is_number(str: &String) -> bool {
        let res = str.parse::<i32>();
        if res.is_ok() {
            return true;
        } else {
            return false;
        }
    }
}
