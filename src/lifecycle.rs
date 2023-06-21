use std::fmt::Display;

pub struct Lifecycle<'a> {
    part: &'a str,
}

// 借用检查器会比较变量的生命周期，而非作用域
// 实际上每个引用都有生命周期
// 生命周期标准只是描述了多个引用生命周期的关系，但不影响实际的生命周期

// &i32 => &'a i32 => &'a mut i32

// a生命周期是a,b中实际生命周期较小的那个，所以返回值也是这个较小的生命周期，如果外部代码实际的生命周期比该生命周期长，并且接受其返回值的话，借用检查器报错
fn largest<'a>(a: &'a i32, b: &'a i32) -> &'a i32 { // 声明了一个声明周期，跟泛型一样，泛型式声明了一个类型。
    return a;
}

impl<'a> Lifecycle<'a> {
    fn life(arg: &'a str) -> &'a str {
        "hello"
    }

    fn method_no_omit_lifecycle(&'a self) -> i32 {
        3
    }

    fn method(&self) -> i32 {
        3
    }
}

fn difficult<'a, T>(x: &'a str, y: &'a str, z: T) -> &'a str
where 
    T: Display
    {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }