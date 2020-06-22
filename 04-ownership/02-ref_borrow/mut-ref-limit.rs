let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;

println!("{}, {}", r1, r2);

// cannot borrow `s` as mutable more than once at a time
// 在特定作用域中的特定数据有且只有一个可变引用。