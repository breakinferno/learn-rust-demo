
fn test_basic() {
    let x = return_str(); // 函数中x move到main函数中，x有意义
    
    let ss = String::from("hell");
    let ret = return_org(ss); // ss move到函数参数，导致ret有意义，ss 无意义
    println!("{}", ss); // 报错
}

fn return_str() -> String {
    let x = String::from("haha");
    x
}

fn return_org(str: String) -> String {
    String::from("mdzz")
}

#[derive(Debug, Copy, Clone)]
struct A {
    a: i32,
    b: Box<i32> // 成员为move语义，结构体和枚举类似也不会实现copy语义
}

// 但元组默认实现copy语义

fn test() {
    let a = A {a: 1, b: Box::new(2)};
    let b = a; // 如果struct A 成员有move语义则不能实现copy语义
    println!("{}", a); // 如果struct A 没实现copy clone trait则报错
}

fn test_vec() {
    println!("Hello, world!");
    let v = [1,2,3]; // vec这里是copy 语义
    foo(v);
    let v2 = v;
    println!("{:?}", v); // 1 2 3
    assert_eq!([1,2,3], v);
}

fn foo(mut v:[i32;3]) -> [i32;3] {
    v[0] = 3;
    assert_eq!([3,2,3], v);
    v
}

fn test_mutable_ref() {
    
    let mut i = 20;
    let a = &i;
    let b = &mut i;
    // println!("{}", a);
    *b = 3;
    compute(&i, &mut i); // 报错，类似于上面注释代码，可变引用后是独占排他锁 https://rust-book.junmajinlong.com/ch6/04_understand_mutable_ref.html
}

fn compute(input:&i32, output:&mut i32) {
    
}


// 解引用操作会改变所有权
fn test_deref() {
    let v = &vec![11, 22];
    let vv = *v; // 报错 可以这样理解，v指向实际具有vector所有权的一个临时值，该值是vector的唯一owner，v不是vector的owner, 但是解引用会产生一个新的临时值，想将该临时值所有权交给vv，这就办不到了，因为v无权做这件事
}