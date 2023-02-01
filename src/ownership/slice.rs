/**
 * 切片是一种不持有所有权的数据类型，语法记作&str，它允许我们引用集合中某一段连续的元素序列，而不是整个集合。
 * 感觉和py的切片是类似的，只是语法不同而已
 */
fn slice() {
    let str = String::from("slice test");
    let slice = &str[0..5];
    let test = &str[6..];
    println!("{}", slice);
    println!("{}", test);
}

fn main() {
    let s = String::from("hello slice can you hear me?");
    let str_const = "const str"; //字符串字面量本身就是切片类型
    let res = find_first_word(&s);
    println!("{}", res);
}

//函数的入参使用了切片类型&str，可以同时处理String和&str类型，更加通用；因为&String
fn find_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &ch) in bytes.iter().enumerate() {
        if ch == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}
