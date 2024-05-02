fn main() {
    let mut s = String::from("hello");
    s.push_str(", world");

    println!("{}", s);

    // let x = 5;
    // let y = x;
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1={}, s2={}", s1, s2);

    let s = String::from("hello");
    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("len of {} is {}", s1, len);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}
fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
fn calculate_length(s: &String) -> usize {
    s.len()
}
