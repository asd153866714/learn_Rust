fn main() {
    let x = plus_one(5);

    println!("The value of x is: {}", x);
}

// 指定傳回值的型態
// 相當於 return
fn plus_one(x: i32) -> i32 {
    x + 1
}