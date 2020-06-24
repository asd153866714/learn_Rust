// 費式數列
use std::io;

fn fib(n: u32) -> u64 {
    if n==0 || n==1 {
        return 1;
    }
    else {
        return fib(n-1) + fib(n-2);
    }
}

fn main() {
    println!("-----Fibonacci numbers Using Recursive-----");

    loop {
        println!("Input a number:");
        let mut n = String::new();

        io::stdin().read_line(&mut n)
            .expect("Failed to read line");

        let n: u32 = match n.trim().parse(){
            Ok(num) => num,
            Err(_) => break,
        };

        println!("result : {}\n", fib(n));
    }

}