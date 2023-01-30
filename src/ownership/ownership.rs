fn main() {
    let s1 = String::from("hello");
    let s2 = s1; //所有权从s1转移到s2，并且s1的所有权无效
    println!("{} world", s1);
}
