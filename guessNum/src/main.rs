extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("猜数字哦!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("目标数字是:{}", secret_number);
    loop {
        println!("请输入你猜测的数字");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("读取失败");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你输入的是:{}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("猜小了"),
            Ordering::Greater => println!("猜大了"),
            Ordering::Equal => {
                println!("猜对了,真厉害");
                break;
            }
        }
    }
}
