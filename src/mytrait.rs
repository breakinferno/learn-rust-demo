// trait and trait bounds
// 接口
// 类型标记
// 泛型限定  单态化-> 静态派遣
// 抽象类型（trait object) 特征对象：动态派遣 动态语言调用 dyn Trait(类似实现多态) 如果使用泛型，其实只能实现一种类型，不能混合，比如我想要一个动物vec, 里面有人，猫，狗，泛型就只能
// 有人或者猫或者狗，dyn Animal 就可以混合

use std::{fmt::{Display, Debug}};

pub trait Summary {
    fn no_default(&self) -> ();
    fn summarize(&self) -> String {
        self.no_default(); // 默认实现
        String::from("summarize default trait output")
    }
}

pub struct Article {

}

impl Summary for Article {
    fn summarize(&self) -> String {
        return String::from("32");
    }
    fn no_default(&self) -> () {
        println!("no default trait fn");
        ()
    }
}

// trait 作为参数
pub fn notify(item: impl Summary, item2: impl Summary) {
    println!("Breaking news {}", item.summarize());
    println!("Breaking news {}", item2.summarize());
}
// trait bounds 写法
pub fn notify_trait_bounds<T: Summary>(item: T, item2: T) {
    println!("Breaking news {}", item.summarize());
    println!("Breaking news {}", item2.summarize());
}

pub fn notify_multi_trait(item: impl Summary + Display) {

}

pub fn notify_multi_trait_return(item: impl Summary + Display) -> impl Summary {
    Article{}
}


pub fn notify_multi_trait_bounds<T: Summary + Display>(item: T) {
    
}

pub fn notify_multi_trait_bounds_where<T, U>(item: T, key: U) 
where 
    T: Summary + Display,
    U: Clone + Debug,
{
    
}