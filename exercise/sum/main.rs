// 計算 0~10 總和
fn main() {
    let mut sum = 0;
    for n in 0..10 {
        sum += n;
        println!("{}",sum);
    }
    println!("sum of 1..10 = {}", sum);
}
