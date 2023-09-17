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

fn matchOption() {
    let a = 6;
    let b: Option<i32> = Some(3);
    //Option是一个枚举类型，在使用时需要先判断是否为None
    match b {
        Some(i) => {
            println!("{}", a + i);
        }
        None => (),
    };
    //if let语句可以让我们只关心一种匹配而忽略其他匹配情况。else语句则相当于match语句中_=>{}通配符的情况。
    //语法：
    //if let 模式 = 表达式/变量 {}
    //else {}
    if let Some(i) = b {
        println!("{}", i);
    } else {
        println!("None");
    }
}
