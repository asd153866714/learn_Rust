fn first_world(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..1];
        }
    }

    &s[..]
}

fn main() {
    let mut s = String::from("hello world");

    let word = first_world(&s);

    s.clear();  // 错误!

    println!("the first word is : {}", word);
}