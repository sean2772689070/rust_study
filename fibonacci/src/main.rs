use std::io;
fn main() {
    println!("请输入要求的斐波那契数列的阶数：");
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("输入错误");
    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("输入类型不是数字, 请重新输入");
            return;
        }
    };
    if n == 1 || n == 2 {
        println!("斐波那契数列为：1");
    } else {
        let mut i = n;
        let mut tmp = 0;
        let mut x = [1, 1];
        while i > 2 {
            tmp = x[0] + x[1];
            x[0] = x[1];
            x[1] = tmp;
            i -= 1;
        }
        println!("{}阶斐波那契数列为：{}", n, tmp);
    }
}
