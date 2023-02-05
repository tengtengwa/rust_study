/**
 * rust的结构体类似于c/cpp的结构体，都表示一个对象相关的数据，是一种数据的组织形式
 */
fn main() {
    let ming = Student {
        age: 18,
        name: String::from("小眀"),
        level: String::from("高三6班"),
    };
    println!("{},{},{}", ming.name, ming.age, ming.level);
    //let创建的结构体实例无法修改：
    // ming.age = 16;
    //想要创建可变的结构体实例，可以通过let mut来声明：
    let mut hong = Student {
        name: String::from("ttw"),
        level: String::from("高三3班"),
        age: 17,
    };
    hong.age = 18;
    hong.name = String::from("小红");

    let wang = fun_return_student(String::from("wang"), 20);
}

//一个通过传入参数创建结构体实例的函数
fn fun_return_student(name: String, age: i32) -> Student {
    Student {
        name: name,
        age, //当字段名和变量名相同时，可以使用简化写法
        level: String::from("高三1班"),
    }
}

struct Student {
    name: String,
    age: i32,
    level: String,
}
