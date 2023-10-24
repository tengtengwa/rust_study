// fn main() {
//     let str1 = String::from("abcd");
//     let result;
//     {
//         let str2 = String::from("xyz");
//         result = get_longest(str1.as_str(), str2.as_str());
//     }
//     println!("{}", result);
// }

use std::fmt::Display;

fn main() {
    //生命周期较短的变量可以引用生命周期比它长的对象，但反过来不行
    // let r;
    // {
    //     let x = 6;
    //     r = &x;
    // }
    // println!("{}", r);

    //函数使用生命周期参数
    let a = &6;
    let result;
    {
        let b = &66;
        result = get_largest(a, b);
    }
    println!("{}", result);

    //结构体声明生命周期参数
    let str = "shabi";
    let p1 = Person { name: str };

    let res = print_string_and_get_longest("str1:666", "str2:8", "str to print");
    println!("{} is longer", res);
}

/**
 * 任何引用都有自己的生命周期。
 * 从根本上来说，生命周期用于关联一个函数中不同参数和返回值的生命周期，以避免出现悬垂引用等内存安全问题
 */

/**
 * 1、生命周期类型的标注：
 * &i32         //引用
 * &'a i32      //显示声明生命周期的引用
 * &'a mut i32  //显示声明生命周期的可变引用
 */

//2、通过泛型标注参数以及返回值的生命周期：
//例如这里声明了泛型生命周期参数'a，表明传入函数的str1和str2以及返回值三者的生命周期是一致的，并且都不短于'a。
//'a在这里只是生命周期的形参，具体来说就是表示str1、str2的生命周期中较短的那个。
fn get_longest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

fn get_largest<'a>(x: &'a i32, y: &'a i32) -> &'a i32 {
    if x > y {
        x
    } else {
        y
    }
}

//3、结构体定义中声明生命周期参数
//这个生命周期参数表明结构体实例的存活时间不超过name这个引用的存活时间
struct Person<'a> {
    name: &'a str,
}

//4、生命周期省略
//编译器会使用下面三条规则来计算引用的生命周期：
//- 函数中每一个引用类型都有自己的一个生命周期参数，例如：foo<'a>(x:&'a str)，foo<'a,'b>(x: &'a str, y: &'b str)
//- 当只有一个输入生命周期参数时，这个生命周期会被赋予给所有输出生命周期参数：foo<'a>(x:&'a str)-> &'a str
//- 当有多个输入生命周期参数时，而其中一个是&self/&mut self时，self的生命周期会被赋予给所有输出生命周期参数

//eg1：结合第一二条，这个函数的生命周期可以被推导为 get_word<'a>(words: &'a str) -> &'a str
fn get_word(words: &str) -> &str {
    return words;
}

//eg2：结合1、3条，这个方法可以推导为 get_age<'a>(&'a self) -> &'a i32
impl<'a> Person<'a> {
    fn get_age(&self) -> &i32 {
        &6
    }
}

//5、静态生命周期
//“Rust中还存在一种特殊的生命周期'static，它表示整个程序的执行期。所有的字符串字面量都拥有'static生命周期”
fn method() {
    let s: &'static str = "666";
}

//6、泛型参数、trait约束和生命周期同时使用
//生命周期是泛型的一种，所以它们可以一起使用
fn print_string_and_get_longest<'a, T: Display>(str1: &'a str, str2: &'a str, str: T) -> &'a str {
    println!("{}", str);
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}
