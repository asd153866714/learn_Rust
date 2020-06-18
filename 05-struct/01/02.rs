struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1
    }
}

fn main() {
    let user1 = build_user(String::from("someone@example.com"), String::from("someusername123"));
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from(("anotherusername567")),
        // ..user1 
        active: user1.active,
        sign_in_count: user1.sign_in_count
        
    }    
}

//「結構更新語法」，可以直接以現有的相同結構實體部份
// 或全部的欄位值來產生新的結構實體。
// 第22行和第23行可以直接簡化成 ..user1