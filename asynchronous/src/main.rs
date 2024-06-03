use std::{thread, time::Duration};
fn main() {
    creating_thread();
    println!();
    stop_restart_thread();
    println!();
    exchange_data_thread();
    println!();
}

fn creating_thread() {
    let handle = thread::spawn(|| {
        println!("Hello Thread!");
        "🍣".to_string() + "🍺"
    });
    let s = handle.join().unwrap();
    println!("{}", s);
}

fn stop_restart_thread() {
    let handle = std::thread::spawn(move || {
        print!("hello");
        thread::park(); // スレッドの停止
        println!("world");
    });
    thread::sleep(Duration::from_secs(1)); // 1秒待機
    println!("Hello World");
    handle.thread().unpark(); // スレッドの再開
    handle.join().unwrap();
}

fn exchange_data_thread() {
    let x = "Hello World".to_string();
    let mut y = "Hello Rust".to_string();
    let rev_x = thread::scope(|scope| {
        let handle1 = scope.spawn(|| x.chars().rev().collect::<String>());
        let handle2 = scope.spawn(|| {
            y += &x;
            y += "Tutorial";
        });
        let rev_x = handle1.join().unwrap();
        handle2.join().unwrap();
        rev_x
    });
    println!("y is :{},rev_x is :{}", y, rev_x)
}
