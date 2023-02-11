/**
 * rust中，方法和函数是两个不同的概念。方法总是被定义在结构体、枚举类型、trait对象中，并且他们的第一个参数永远都是self，
 * 也就是说方法总是和对象相关联。
 */
fn main() {
    let r1 = Rect {
        height: 30.0,
        width: 50.0,
    };
    // println!("area：{}, perimeter：{}", r1.cal_area(), r1.cal_perimeter());
    let r2 = Rect {
        height: 10.0,
        width: 40.0,
    };
    let r3 = Rect {
        height: 60.0,
        width: 45.0,
    };
    println!("r1能包含r2吗：{}", r1.can_hold(&r2));
    println!("r1能包含r3吗：{}", r1.can_hold(&r3));
    println!("我要一个正方形：{:#?}", Rect::square(6.0)); //关联函数可以通过“类名::关联函数名”的形式调用。
}

#[derive(Debug)]
struct Rect {
    height: f32,
    width: f32,
}

impl Rect {
    //方法签名&self实际上被推导为了rect:Rect
    fn cal_area(&self) -> f32 {
        return self.height * self.width;
    }

    fn cal_perimeter(&self) -> f32 {
        return self.height * 2.0 + self.width * 2.0;
    }

    fn can_hold(&self, other: &Rect) -> bool {
        if self.height > other.height && self.width > other.width {
            return true;
        }
        if self.height > other.width && self.width > other.height {
            return true;
        }
        return false;
    }

    //关联函数，与结构体相关联，但没有self函数
    fn square(size: f32) -> Rect {
        Rect {
            height: (size),
            width: (size),
        }
    }
}
