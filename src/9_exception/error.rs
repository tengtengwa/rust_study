use std::{
    fs::File,
    io::{self, ErrorKind, Read},
};

fn main() {
    //不可恢复错误及panic：
    let arr = vec![1, 2, 3];
    // arr[66];
    // panic!("手动抛出panic~");

    //可恢复错误及Result
    let path = "/Users/tengtengwa/GithubProjects/rust_study/src/test.java";
    let f = File::open(path);
    match f {
        Ok(file) => {
            println!("成功打开文件");
        }
        Err(error) => {
            println!("打开文件失败");
            match error.kind() {
                //unwrap()方法会返回Result枚举中OK的值，如果枚举中是Err，则直接panic，不推荐直接这样使用。
                //File::create(path).unwrap().expect("failed to create file");
                ErrorKind::NotFound => match File::create(path) {
                    Ok(f) => {
                        println!("create file successfully");
                    }
                    Err(e) => {
                        println!("create file failed");
                    }
                },
                _ => {
                    println!("other error occur")
                }
            }
        }
    }
}

/**
 * 传播错误，类似于java中方法抛出异常，交给调用者处理异常。
 */
fn spread_panic() -> Result<String, io::Error> {
    let f = File::open("hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn simplified_spread_panic() -> Result<String, io::Error> {
    let mut s = String::new();
    //?运算符在表达式返回Ok时继续执行程序，在表达式返回Err时break当前函数，并将Err传播给调用者
    File::open("hello.txt")?.read_to_string(&mut s)?;
    //最后一行的返回值可以不写return，但也不能在语句后加分号，否则返回值就是空类型()。
    Ok(s)
    // return Ok(s);
}
