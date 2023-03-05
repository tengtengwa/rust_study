fn main() {
    let old_add = IpAddr::V4(192, 168, 0, 0);
    let new_add = IpAddr::V6(String::from("abcdefg"));
}

enum IpAddr {
    V4(u8, u8, u8, u8), //枚举类型的变体可以和任意数据关联
    V6(String),
}

enum Message {
    Content(String),
    Title(String),
    Token,
    TimeStamp { startTime: i64, upDateTime: i64 },
}

//枚举类型也可以拥有方法
impl Message {
    fn getContent(&self) -> &str {
        return "I am content";
    }
}
