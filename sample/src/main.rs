use rand::Rng;

fn main() {
    let a = rand::thread_rng().gen_range(1..=100);
    let b = rand::thread_rng().gen_range(1..=100);

    let product = a * b;

    if product % 2 == 0 {
        println!("a:{}とb:{}の積は{}で、偶数です。", a, b, product);
    } else {
        println!("a:{}とb:{}の積は{}で、偶数ではありません。", a, b, product);
    }
}
