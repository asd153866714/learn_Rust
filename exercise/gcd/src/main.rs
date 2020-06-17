// 找最大公因數
use std::io;
use text_io::scan;

fn gcd(a:u32, b:u32) -> u32 {
    let result : u32 = if b==0 {a} else {gcd(b, a%b)};
    return result;
}

fn main() {
    println!("------found gcd(a, b)------\n");
    println!("Enter two number ex: 10 20 :");

    let (a, b): (u32, u32);
    scan!("{} {}", a, b);

    println!("gcd of (a, b) = {}",gcd(a, b));
}
