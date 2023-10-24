use std::fmt::Display;

fn main() {
    let p1 = Map { key: 1, value: 6 };
    let p2 = Map { key: 1, value: "2" };
    let p1Key = p1.getKey();
    let p2Value = p2.getValue();
    println!("{}", p1.to_string());
    println!("{}", p1.default_print());
    p2.print_trait(Map { key: 2, value: "2" });
    p2.print_trait_use_generic(Map { key: 3, value: "3" });
}

//结构体声明泛型和Java中类的泛型也很像，都是在类名后声明泛型参数
struct Map<K, V> {
    key: K,
    value: V,
}

//方法的声明需要注意，如果方法也需要使用泛型，则需要在impl关键字后也声明泛型参数，否则就需要在结构体后使用具体的参数类型。
impl<K, V> Map<K, V> {
    fn getKey(&self) -> &K {
        return &self.key;
    }

    fn getValue(&self) -> &V {
        return &self.value;
    }

    //方法也可以有自己声明的泛型参数
    fn getMixMap<T, U>(self, other: Map<T, U>) -> Map<K, U> {
        Map {
            key: self.key,
            value: other.value,
        }
    }

    //trait也可以作为参数，需要在对象的类型中使用"impl trait名"的形式。这一点和接口也比较类似
    fn print_trait(&self, item: impl Print) -> () {
        println!("{}", item.to_string());
    }

    //也可以通过泛型的形式将trait作为参数
    fn print_trait_use_generic<T: Print>(&self, item: T) -> () {
        println!("{}", Print::to_string(&item));
    }

    //通过+语法可以指定多个trait约束
    fn multi_trait<T: Print + ToString>(&self, item: T) -> () {
        println!("{}", ToString::to_string(&item));
    }

    //trait同样可以作为返回值，格式："impl trait名"
    //注意：只能通过impl Trait返回一种实现这个Trait的类型，如果通过if语句返回两种实现类型，则会无法通过编译。
    fn return_trait(&self) -> impl Print {
        Map { key: 6, value: "6" }
    }
}

//为类型实现trait：impl 实现的trait名 for 类型
impl<K, V> Print for Map<K, V> {
    fn to_string(&self) -> &str {
        "Map的to_string trait"
    }
}

impl<K, V> ToString for Map<K, V> {
    fn to_string(&self) -> String {
        String::from("ToString Trait for Map")
    }
}

//声明方法时不使用泛型参数，表明只为具体的一种结构体类型声明方法。例如下面声明的这些方法只能用于Map<i32, i32>这一种类型
impl Map<i32, i32> {
    fn getIntKey(&self) -> &i32 {
        return &self.key;
    }

    fn getIntValue(&self) -> &i32 {
        return &self.value;
    }
}

//trait的声明和接口类似，trait+trait名
pub trait Print {
    fn to_string(&self) -> &str; //其中的方法仅声明，不实现。和接口中声明抽象方法类似

    //trait中声明的方法也可以有默认实现，实现此trait的类型同样也可以重写此方法
    fn default_print(&self) -> &str {
        return "print的默认实现";
    }
}
