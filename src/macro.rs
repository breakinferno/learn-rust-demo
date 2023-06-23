struct MM<T>(T);

#[macro_define_crate::first_attr_proc_macro("some args")]
fn add(a: i32, b: i32) {
    println!("hewhe  {}", a + b);
}