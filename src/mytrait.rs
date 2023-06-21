// trait and trait bounds

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