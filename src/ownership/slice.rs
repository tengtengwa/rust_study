/**
 * 切片是一种不持有所有权的数据类型，它允许我们引用集合中某一段连续的元素序列，而不是整个集合。
 * 感觉和py的切片是类似的，只是语法不同而已
 */
fn slice() {
    let str = String::from("slice test");
    let slice = &str[0..5];
    let test = &str[6..];
    println!("{}", slice);
    println!("{}", test);
}

fn find_first_word(s: &String) -> &str {    //注意函数返回字符串切片的写法是&str
    let bytes = s.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}

