fn get_word(s: &str) -> &str {

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..2];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("test");

    let my_str = "hello";

    let word = get_word(my_str);

    println!("{}", word);
}