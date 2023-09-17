fn main() {
    let s = "666";
    //to_string()实际上也是通过调用String::from()来创建String对象的
    let s1 = s.to_string();
    let mut s2 = String::from("888");
    //push方法没有返回值，为什么能被s3引用到的呢？而且s3的类型是()是什么意思？
    let s3 = s2.push('6');
    //+拼接字符串，实际上是调用了类似于这样一个函数：fn add(self, s: &str) -> String。此时s1的所有权移动到了s4
    let s4 = s1 + &s2;
    //创建字符串切片：和Python类似的语法，切片区间也是[start,end)，索引表示起始和结束的字节，而不是字符
    let s5 = "str".to_string() + &s2[0..2];
    //format!宏用于格式化输出字符串
    let s6 = format!("{},{},{}", s, s4, s2);

    let str = "Здравствуйте"; //打印[0,4)范围的字节/切片，结果是Зд
    let str2 = "123456"; //打印[0,4)范围的字节/切片，结果是1234

    //遍历字符串中的字符：str的bytes()方法类似于java中String.toCharArray()方法，通过for循环即可遍历
    //str还有chars()方法，用于将字符串转换为Unicode字符数组
    for ch in str.bytes() {
        print!("{} ", ch);
    }
}
