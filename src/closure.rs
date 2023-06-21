pub fn test_closure() {
    let i = 3;
    let infn = || i + 4;
    infn();
}

// 函数指针
fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn test_fn_pointer() {
    let mut x = add(5,7);

    type Binop = fn(i32, i32) -> i32;
    let bo: Binop = add;
    x = bo(5,7);
}



// 闭包
struct  Cache<T>
where
    T: Fn(i32) -> i32,
    {
        func: T,
        value: Option<i32>
    }

impl<T> Cache<T>
where
    T: Fn(i32) -> i32,
{
    fn new(method: T) -> Cache<T> {
        Cache {
            func: method,
            value: None
        }
    }

    fn value(&mut self, arg: i32) -> i32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.func)(arg); // 为啥要括号括起来呢？
                self.value = Some(v);
                v
            }
        }
    }
}


fn add_once(x: i32) -> i32 {
    x + 1
}

fn do_more(f: fn(i32) -> i32, arg: i32) -> i32 { // fn 表示函数指针类型
    f(arg)
}

fn do_twice<T>(f: T, arg: i32) -> i32
where 
    T:Fn(i32) -> i32 // Fn 表示闭包特征限制， 既可以接受闭包，也可以接受函数指针，C++就不能接受闭包
{
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn return_closure() -> impl Fn(i32) -> i32 { // 返回闭包需要闭包特征
    |x| x+3
}

fn return_closure_pro(a: i32) -> Box<dyn Fn(i32) -> i32> { // 返回闭包需要闭包特征
    if a> 0 {
        Box::new(move |x| x+a)
    } else {
        Box::new(move |x| x-a)
    }
}

fn test() {
    do_twice(add_once, 3);

    let list_of_statues: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

// 教程： https://www.bilibili.com/video/BV1Zf4y1J7jq/?spm_id_from=333.337.search-card.all.click&vd_source=05e080b508420a654337f2995526fa22
// https://github.com/dongzerun/rust_closure