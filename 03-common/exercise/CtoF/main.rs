// 攝氏轉華氏
use std::io;

fn main() {

    println!("Input C :");

    let mut c = String::new();

    io::stdin().read_line(&mut c)
        .expect("Failed to read line");
    
    let c: u32 =  c.trim().parse()
        .expect("test");

    let f = c * 9 / 5 + 32;

    println!("result F : {}", f);
}