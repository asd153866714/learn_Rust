fn E() {
    println!("E started");

    F();

    println!("E finished");
}

fn F() {
    println!("  F started");
    println!("      F");
    println!("  F finished");
}

fn main() {
    E();
}