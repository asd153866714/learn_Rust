fn main() {
    let s = String::from("test");
    string_move(s);
    // println!("{}", s);      value borrow after move

    let x = 5;
    int_copy(x);
    println!("{}", x);
}

fn string_move(s: String){
    println!("{}", s);
}

fn int_copy(x: i32){
    println!("{}", x);
}