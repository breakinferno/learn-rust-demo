// cargo test 默认行为， 并行运行所有测试，捕获（不显示）所有输出
// 并行测试： 确保不共享内容  cargo test -- --test-threads=1
// 显示函数输出：失败的测试才会打印println!这种信息， --show-output参数解决
// 测试子集 
// 单个测试 cargo test your_test_name
// 多个测试 可以使用模块 或者 模糊匹配名字

#[cfg(test)] // 这里好像没用 只有运行cargo test才会编译和运行 cargo build 不会
#[test]
fn it_works() {
    assert_eq!(2+2, 4);
}

#[cfg(test)] // 测试模块
mod inner {
    fn return_four() -> i32 {
        4
    }

    #[test]
    #[ignore = "too expensive"] // cargo test -- --ignored 单独运行忽略的测试
    fn some_expensive_method() {

    }


    #[test]
    fn it_doesnot_work() {
        assert!(return_four()-4 == 0); // 为true ok， 为false则 panic
    }

    #[test]
    fn it_doesnot_work_customize() {
        assert!(return_four()-3 == 0, "This is custom error info"); // 为true ok， 为false则 panic
    }
}

#[test]
#[should_panic]
fn test_should_panic() {
    let v = vec![2];
    v[2];
}

#[test]
#[should_panic(expected = "index out of bounds")]
fn test_should_panic_expected() {
    let v = vec![2];
    v[2];
}

#[test]
fn test_result() -> Result<(), String> {
    if 2+2==4 {
        Ok(())
    } else {
        Err(String::from("failed"))
    }
}