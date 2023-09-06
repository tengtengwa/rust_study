/**
 * 引用与借用
 */
fn main() {
    let s1 = String::from("hello");
    let s2 = calculate_len(&s1);
    println!("{}", s1);
}

//函数str参数的类型声明为&String，&就是引用的语义，表示允许我们在不获取所有权的情况下使用其值
//通过引用传递参数给函数的方法就叫做“借用”
fn calculate_len(str: &String) -> usize {
    str.len()
}

//rust中引用默认是不可变的，rust不允许我们修改引用指向的值；如果需要修改，则可以使用可变引用
fn mut_reference() {
    //可变引用的声明：let mut
    let mut s = String::from("mut str");
    let a1 = &s;
    let a2 = &s;
    let a3 = &mut s; //√ 可变引用可以指向一个可变引用(这里没有显示指定a3的类型，编译器推导为&mut String类型)
    //× 但不可变引用不能指向一个可变引用；同理，可变引用也不能指向一个不可变引用，可以理解为类型不匹配
    let a4: String = &mut s;
    let a5: &mut String = &s;
    //调用函数传递可变引用类型时使用&mut
    change(&mut s);
    println!("{}", s);
}

//函数中可变引用类型的参数的声明：&mut
fn change(str: &mut String) {
    str.push_str(",apppend");
}
