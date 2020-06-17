// 成績分級
use std::io;

fn main() {
    
    println!("Please enter your grades :");

    let mut a = String::new();

    io::stdin().read_line(&mut a)
        .expect("Failed to read line");

    let a: u32 = match a.trim().parse(){
        Ok(num) => num,
        Err(_) => panic!("Faild"),
    };

    if a > 80 {
        println!("A");
    }
    else if a > 70 {
        println!("B");
    }
    else if a > 60 {
        println!("C");
    }
    else {
        println!("D");
    }

}
