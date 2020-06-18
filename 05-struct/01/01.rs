// 定義結構
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

// 宣告結構實例

// 如果想要讓結構體的某些欄位在實體化時是選填的話，
// 可以透過實作一個建立出該結構體實體的函數來供以後重複使用。
fn build_user(email: String, username: String) -> User {
    User {
        email,              // 當變數名稱和欄位名稱相同的時候，可以省略撰寫欄位名稱，                          
        username,           // 直接填入變數名稱即可。
        active: true,
        sign_in_count: 1
    }
}
 
fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
}