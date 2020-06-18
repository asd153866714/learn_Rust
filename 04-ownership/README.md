# Rust 的 擁有權

* 執行程式的時候，需要管理其使用記憶體的方式

    1. 程式設計師自己分配和釋放：C
    2. 垃圾回收機制：javascript
    3. 擁有權系統：Rust
# 堆疊(Stack) 和 堆積(Heap)

* 堆疊：不用考慮內存空間的分配，存取速度較快，但是資料需要占用固定的大小。

* 堆積：記憶體區塊較自由，用來存放未知大小的資料，利用指標紀錄位址來分配內存空間。

* 透過擁有權的機制，可以避免「記憶體洩漏」的情形

# 所有權規則
1. 每個Rust中的值所對應的變數，稱作該值的「擁有者」(owner)。
2. 同一時間，每個值只能分別有一個「擁有者」。
3. 當離開作用域(scope)時，屬於該 scope 底下的「擁有者」所持有的值都會被消滅

# 作用域 (scope)
```
fn main(){
    let a = 1;
}
```
變數 a 在函數結束後，原本對應的值 1 會從堆疊中被刪除，也就是 Stack 中 pop 的概念。

# String 類型

`String` 是 Rust 內建的一種結構體，可以用來儲存輸入的文字，但是在程式執行階段，無法知道使用者輸入的資料長度，因為資料長度不確定，會儲存在 Heap。

# 內存分配
```
{
    // 從這裡開始，s 是有效的
    let s = String::from("hello"); 

    // 使用 s

}  // 作用域結束，s 不再有效

```
# 複製 (copy)
```
let x = 5;
let y = x;

println!("x = {}, y = {}", x, y);
```
變數為整數型態，是已知固定大小的值。

# 移動 (move)
```
fn main() {
let s1 = String::from("hello");
let s2 = s1;

println!("{}", s1);
}
```
會發生錯誤，因為 `String` 是結構，不能用複製的方式指派變數的值，而是用移動的方式，s1 的值移動給 s2，因此 s1 不再有效，
"hello" 的擁有者也從 s1 變成 s2 。

# 克隆 (clone)
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
 
    println!("{}", s1);
}
```

# 擁有權和函數

* 呼叫函數傳遞參數的方式和前面類似：
    
    * copy： 參數為固定大小的型態
    * move： 相反
```
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效
    // println!("{}", s);                              

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x
    // println!("{}", x);

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作
```
* `println!("{}", s)` 會發生錯誤，因為上面的 s 已經被移動給`takes_ownership(s)`，所有權也被轉移了。

* `println!("{}", x)` 不會發生錯誤，因為 x 是固定大小的整數型態，被複製給`makes_copy(x)`，不會改變原有的值和所有權。


# 引用與借用
```
fn main() {
    let s = String::from("hello");
 
    let len = calculate_length(&s);
 
    println!("The length of '{}' is {}.", s, len);
}
 
fn calculate_length(s: &String) -> usize {
    s.len()
}
```
* `&` 符號代表「引用」(reference)，`&s`創建了一個指向`s`的值的引用，但是沒有所有權，作用域結束後也不會丟棄引用的值。

* 我們將獲取引用作為函數參數稱為「借用」(borrowing)，使用完畢，必須歸還。

# 可變引用

```
fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```
* 會發生錯誤，正如一般的變數，引用的變數也是預設不可變的，必須加上`mut`。
```
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

* 為了維持程式的安全性，使用可變引用有個很大的限制，一個變數在一個 scope 下，同時只能夠有一個可變引用，例如：
```
	fn main() {
    let mut s = String::from("hello");
 
    let r1 = &mut s;
    let r2 = &mut s;
}
```
`r2` 會發生錯誤，因為`s`已經有一個可變引用`r1`。

* Rust 這樣的設計是為了避免「資料競爭」(data race)，可由這三種行為產生：

    1. 有兩個或更多的指標同時存取同一筆資料。
    2. 有一個以上的指標正在被用來寫入資料。
    3. 沒有同步訪問資料的機制。

</br>

可以使用大括号来創建一个新的作用域，以允许擁有多个可變引用，只是不能 **同時**擁有：
```
let mut s = String::from("hello");

{
    let r1 = &mut s;

} // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

let r2 = &mut s;
```

</br>

在一個scope下，不可變參考和可變參考也不能同時使用。舉例來說：
```
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
let r3 = &mut s; // BIG PROBLEM
```
</br>

可變的變數被借給其它變數使用之後，在尚未歸還前，都不能再被改變。舉例來說：
```
fn main() {
    let mut a = 1;
    let b = &a;
    // a = 1;
    println!("a = {}", a);
}
```
