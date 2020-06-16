fn main() {
    // 用模式匹配（pattern matching）来解构（destructure）元组值
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // x.1 = x[1]
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("The value of one is :{}", one);
}