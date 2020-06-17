// 99乘法表

fn main() {
    for i in 1..10 {
        for j in 1..10 {
            let s = i * j;
            println!("{} * {} = {}", i, j, s);
        }
        println!("--------------")
    }
}
