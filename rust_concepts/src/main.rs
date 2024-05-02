use rand::Rng;

fn main() {
    mutable_number();
    println!();
    shadowing();
    println!();
    data_type();
    println!();
    let x = another_function(5, 'h');
    println!("return: {}", x);
    println!();
    control();
}

fn mutable_number() {
    println!("変数と可変性");

    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    println!("定数");
    const MAX_POINTS: u32 = 100_000;

    println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}

fn shadowing() {
    println!("シャドーイング");
    let x = 5;
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {}", x);
    }

    // string型はコピーではなく、moveなのでスコープエラーが出る。.clone()でコピーすることもできる
    // let x = "aaa".to_string();
    // {
    //     let x = x.clone();
    //     println!("The value of x in the inner scope is: {}", x);
    // }

    println!("The value of x is: {}", x);

    let spaces = "   ";
    let spaces = spaces.len();

    println!("The value of spaces: {}", spaces);
}

fn data_type() {
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess:{}", guess);

    println!("浮動小数点型");
    let x = 2.0;
    let y: f32 = 3.0;
    println!("x:{}, y:{}", x, y);

    println!("数値演算");
    // 足し算
    let sum = 5 + 10;
    // 引き算
    let difference = 95.5 - 4.3;
    // 掛け算
    let product = 4 * 30;
    // 割り算
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3;
    // 余り
    let remainder = 43 % 5;
    println!(
        "sum:{}, sub:{}, multi:{}, div1:{}, div2:{}, rem:{}",
        sum, difference, product, quotient, floored, remainder,
    );

    println!("論理値型");
    let t = true;
    let f: bool = false;
    println!("t:{}, f:{}", t, f);

    println!("文字型");
    let c = 'c';
    let z = 'g';
    let heart_eyed_cat = '😻';
    println!("c:{}, z:{}, icon:{}", c, z, heart_eyed_cat);

    println!("タプル型");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("x:{}, y:{}, z:{}", x, y, z);

    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.2;
    let one = x.2;
    println!("five:{}, six:{}, one:{}", five_hundred, six_point_four, one);

    println!("配列型");
    let a = [1, 2, 3, 4, 5];
    // let b = [3; 5]; => [3, 3, 3, 3, 3]
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let first = a[0];
    let second = a[1];

    println!("first:{}, second:{}", first, second);

    let rand = rand::thread_rng().gen_range(0..5);
    let month = rand::thread_rng().gen_range(1..12);

    let selected_number = a[rand];
    let this_month = months[month];
    println!("{} of a was selected", selected_number);
    println!("This month is {}", this_month);
}

fn another_function(value: i32, unit_label: char) -> i32 {
    println!("The measurement is: {}{}", value, unit_label);

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);

    let x = five();
    println!("x:{}", x);

    value + 10
}

fn five() -> i32 {
    5
}

fn control() {
    let num = 3;

    if num < 5 {
        println!("true");
    } else {
        println!("false");
    }

    if num != 0 {
        println!("num was something other than zero");
    }

    if num % 4 == 0 {
        println!("number is divisible by 4");
    } else if num % 3 == 0 {
        println!("number is divisible by 3");
    } else if num % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);

    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("end count = {}", count);

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    println!("while");
    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    println!("element");
    for element in a {
        println!("the value is: {}", element);
    }

    for number in 1..4 {
        print!("{}!", number);
    }
    println!();

    for number in (1..4).rev() {
        print!("{}!", number);
    }
    println!();
}
