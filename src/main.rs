use std::collections::HashMap;
use std::fmt::Display;
use std::fs::File;
use std::io::Read;
use std::io;

use crate::mytrait::Summary;

mod mytrait;
mod demod;
mod lifecycle;
mod tests; // 不引入mod 不会启动测试
mod closure;

#[derive(Debug)]
enum SheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {

    // Vec
    println!("Hello, world!");
    // 
    let mut rarvec: Vec<i32> = Vec::new();
    rarvec.push(32);
    
    println!("hello {}", rarvec.len());


    let vec = vec![
        SheetCell::Int(32),
        SheetCell::Float(64.0),
        SheetCell::Text("Hello world!".to_string()),
    ];

    for i in &vec {
        println!("{:?}", i);
    }

    for i in vec { // This will move vec
        println!("{:?}", i);
    }

    // String 实际式Vec<u8>的封装
    let mut st = String::from("字符串字面值");
    let mut st2 = "字符串字面值".to_string();
    st.push_str("新的");
    let st3 = String::from("加后");
    st2 + &st3;
    // println!("{}", st2); // add 操作移动了st2所有权
    let t3 = String::from("3");
    let fst = format!("{}-{}-{}", "1", "2", t3); // format 不移动 只借用
    println!("{}", fst);
    print!("{}", t3);

    let sl = &st[0..3];
    println!("{}", sl);
    // let sl = &st[0..4];
    // println!("{}", sl); // panic 因为不是按照字符边界进行切割

    for b in st.bytes() {
        println!("{}", b);
    }

    for c in (&st).chars() {
        println!("{}", c);
    }


    // HashMap

    let mut map = HashMap::new();
    map.insert("32".to_string(), 32);

    let teams = vec![String::from("A"), String::from("B")];
    let scores = vec![10, 20];

    let mut map: HashMap<_, _> = teams.iter().zip(scores.iter()).collect();
    for (k,v) in &map {
        println!("{} : {}", k, v);
    }
    let ak = String::from("A");
    let ck = String::from("C");
    map.entry(&ak).or_insert(&32); // entry判断key是否存在
    map.entry(&ck).or_insert(&33);
    for (k,v) in &map {
        println!("{} : {}", k, v);
    }


    // 错误 panic! 处理不可恢复错误，比如数组越界， Result 处理可恢复错误

    // let f = File::open("hello.txt");
    // let f = match f {
    //     Ok(file) => file,
    //     Err(err) => panic!("{:?}", err)
    // };

    // let f = File::open("hello.txt").unwrap();
    // let f = File::open("hello.txt").expect("无法打开文件");
 
    // 泛型
    let s = Point {x: 2, y: 2.1};
    let ss = Point { x: 3, y: 3};
    ss.x(); // ss没有s1方法
    s.x1();

    // trait
    let a = mytrait::Article {};
    println!("{}", a.summarize());

    let d = demod::Demo {x:2};
    //  let d = demod::demo::Demo {x:3}; // not export demo, so it's private


    println!("{}", 3.summarize()); // 因为3实现了display trait， 而我们又为trait实现了Summary trait， 所以3可以调用summarize方法
}

struct Point<T, U> {
    x: T,
    y: U
}

impl<T, U> Point<T, U> {// 针对类型T U实现方法
    fn x(&self) -> &T {
        &self.x
    }
    fn y(&self) -> &U {
        &self.y
    }
}

impl Point<i32, f64> { // 只针对i32,f64实现方法
    fn x1(&self) -> &i32 { // 只有Point<i32, f64>有该方法
        &self.x
    }
}

impl<T: Display + PartialOrd, U> Point<T, U> {
    fn cmp_display(&self) {

    }
}

impl<T: Display> mytrait::Summary for T {
    fn summarize(&self) -> String { // 为所有实现display trait的类型实现了Summary trait的默认实现 ，所以任何实现display trait的类型也都实现了Summary trait...
        String::from("This is blanket implementations fn")
    }
    fn no_default(&self) -> () {
        ()
    }
}

enum MyEnum<T> {
    A(T),
    b(i32),
}

// fn largest<T>(arr: &[T]) -> T {
    
// }

fn test_open_file() -> Result<String, io::Error> {
    let mut s = String::new();
    let mut f = File::open("hello.txt")?;  // 如果open 为err，直接return 该error   File::open("hello.txt").unwrap_or(Err("custom erro?"))
    f.read_to_string(&mut s)?;
    // File::open("hello.txt")?.read_to_string(&mut s)?;
    Ok(s)
}