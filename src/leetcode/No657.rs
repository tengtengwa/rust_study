use std::collections::HashMap;

fn main() {
    let s = Solution::judge_circle(String::from("LLRR"));
    println!("{}", s);
}

struct Solution {}

/**
 * 657. 机器人能否返回原点
 * https://leetcode.cn/problems/robot-return-to-origin/
 */
impl Solution {
    pub fn judge_circle(moves: String) -> bool {
        let mut map = HashMap::new();
        for ch in moves.chars() {
            let value = map.entry(ch).or_insert(0);
            *value += 1;
        }
        if map.get(&'U') != map.get(&'D') || map.get(&'L') != map.get(&'R') {
            return false;
        };
        return true;
    }
}
