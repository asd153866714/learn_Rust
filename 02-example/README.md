# 猜數字

# 說明
```
let secret_number = rand::thread_rng().gen_range(1, 101);
```
`::` => 尋找指定套件或是模組下的項目：

從 `rand` 套件中呼叫 `thread_rng()` 這個函數，`thread_rng()` 回傳一個 `ThreadRng` 結構實體

`.` => 使用結構實體的方法：

使用 `ThreadRng` 結構實體的方法，`gen_range()`
   
`gen_range(1, 101)` 隨機產生 1~100 的整數

```
let mut guess = String::new();
```
宣告 `guess`，來儲存輸入的文字，產生 `String` 結構的實體


```
io::stdin().read_line(&mut guess)
   .expect("Failed to read line");
```
使用 `read_line()` 方法讀取輸入的內容，並傳入前面宣告的 `guess`

`read_line()` 會傳回 Result 列舉的值 :

   * Ok 
   * Err

`expect()` => 如果結果為 Err， 回傳 "Failed to read line"


```
let guess: u32 = match guess.trim().parse(){
   Ok(num) => num,
   Err(_) => continue,
};
```
宣告一個新的整數型態變數 `guess` ，

`.trim()` => 過濾掉輸入時按下 Enter 鍵而產生的 \n 字元，並回傳另一種型別的字串 ── str

`.parse()` => 將字串轉成數值，回傳 `Result` 列舉

`match` => 根據 `parse()` 回傳值判斷下一步：

   * Ok => 代表轉換成功，形成新的 guess 變數
   * Err => 進入下一次 =循環，等待新的輸入
   * 原本是用 `expect()` 方法，但是輸入非數字時會產生錯誤


```
match guess.cmp(&secret_number){
   Ordering::Less => println!("Too small!"),
   Ordering::Greater => println!("Too big!"),
   Ordering::Equal => {
         println!("You win!");
         break;
   }
}
```
比較輸入的數字和隨機產生的亂數的大小

### 執行
```
user@DESKTOP-9VVBDPS MINGW64 /d/Xing/learn_Rust/02-example/guessing_game (master)
$ cargo run
   Compiling guessing_game v0.1.0 (D:\Xing\learn_Rust\02-example\guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.91s             
     Running `target\debug\guessing_game.exe`
Guess the number!
Please input your guess.
5
You guessed: 5
Too small!
Please input your guess.
10
You guessed: 10
Too small!
Please input your guess.
50
You guessed: 50
Too big!
Please input your guess.
40
You guessed: 40
Too big!
Please input your guess.
30
You guessed: 30
Too small!
Please input your guess.
35
You guessed: 35
Too small!
Please input your guess.
38
You guessed: 38
You win!
```
