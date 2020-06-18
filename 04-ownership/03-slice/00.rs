// string slice
fn main () {
    let s = String::from("hello");

    let len = s.len();

    let a = &s[0..len];
    let b = &s[3..];
    let c = &s[..2];
    let d = &s[..];

    println!("a: {} \nb: {} \nc: {} \nd: {}\n", a, b, c, d);
}