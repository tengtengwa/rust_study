fn main() {
    let p1 = Map { key: 1, value: 6 };
    let p2 = Map { key: 1, value: "2" };
    let p1Key = p1.getKey();
    let p2Value = p2.getValue();
    let vec = vec![0, 3, 5, 6, 3, 56];
    println!("{}", getLargest(&vec));
}

fn getLargest<T: Copy + PartialOrd>(arr: &[T]) -> &T {
    let mut max = &arr[0];
    for item in arr.iter() {
        if item > max {
            max = &item;
        }
    }
    max
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
