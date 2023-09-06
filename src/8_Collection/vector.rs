fn main() {
    //空参构造Vec对象时，需要先显示指定变量的类型，通过Vec的new方法构造对象。
    let vec: Vec<i32> = Vec::new();
    //vec!宏用于简化创建Vec时的代码，仅能用于根据我们提供的值来创建Vec；此时可以不指定变量类型，编译器会推导出来
    let mut v2 = vec![0, 1, 2];
    println!("v2原元素：");
    print_vec_nums(&v2);

    //增：push方法
    println!("\nv2中push(3)");
    v2.push(3);
    print_vec_nums(&v2);
    //删：remove(&mut self, index: usize)，也可以使用pop()、pop_front()等
    println!("\nv2 remove(3)");
    v2.remove(3);
    print_vec_nums(&v2);
    //改：vec没有类似Java中List的set方法，只能通过[]访问元素进行修改。
    println!("\nv2将索引0的元素置为-1");
    v2[0] = -1;
    print_vec_nums(&v2);
    //查：使用索引或get方法
    println!("\nv2通过[]查找元素");
    println!("{}", &v2[2]); //这里&[]获取的是元素的引用；当索引超过数最大小时程序会数组越界导致panic。
    //当通过get方法访问超过数组大小的索引时，会返回一个None；当使用索引访问vec元素时，编译器同样会在编译期进行索引检查
    if let Some(i) = v2.get(5) {
        println!("{}", i);
    }
}

fn print_vec_nums<T: std::fmt::Debug>(vec: &Vec<T>) {
    for n in vec.iter() {
        print!("{:?} ", n);
    }
}
