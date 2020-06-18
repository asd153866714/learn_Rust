#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用 impl 來定義並實作 Rectangle 結構的 area 方法
// Rust 允許多個 impl 關鍵字
impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    // 不一定要將第一個參數設為 &self
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width,
            height,
        }
    }
}
 
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
 
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
 
impl Rectangle {
    // 如果結構實體的方法會改變結構實體本身的值，在用 fn 關鍵字定義方法的時候，
    // 第一個參數必須改為 &mut self
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
 
    fn set_height(&mut self, height: u32) {
        self.height = height;
    }
}
 
fn main() {
    let rect1 = Rectangle::new(30, 50);
 
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}