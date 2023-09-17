use std::collections::HashMap;

fn main() {
    //1、map的简单使用
    let str = "112223";
    let mut map = HashMap::new();
    //增/改：insert方法
    for ch in str.chars() {
        //通过entry(key).or_insert(default)判断key是否存在，不存在则插入key-default；
        //or_insert方法返回的是value的引用（类型为&mut Integer），这里解引用后就可以操作value的值
        *map.entry(ch).or_insert(0) += 1;
        //也可以分成两步来写：
        //let count = map.entry(ch).or_insert(0);
        //*count += 1
    }
    //查：get方法
    let key = '2';
    if let Some(i) = map.get(&key) {
        println!("{}-{}", key, i);
    }
    //删：remove方法
    map.remove(&key);
    println!("{:?}", map);

    //2、由键值对元组构成的vec通过collect方法构造Map
    let strArr = vec![String::from("blue"), String::from("yellow")];
    let scoreArr = vec![1, 2];
    //必须显示指定变量的类型，但编译器可以推导出其泛型
    let str2_score: HashMap<_, _> = strArr.iter().zip(scoreArr).collect(); //zip()方法中可以传入vec对象，也可以传入它的迭代器
    println!("{:?}", str2_score);
}
