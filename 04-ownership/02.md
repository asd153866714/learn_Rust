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

<br>

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

可變的變數被借給其它變數使用之後，在尚未歸還前，都不能再被改變或移動。舉例來說：
```
fn main() {
    let mut a = 1;
    let b = &a;
    // a = 1;
    println!("a = {}", a);
}

fn main() {
    let a = String::from("hi");
    let b = &a;
    let c = a;
}
```

# 垂懸引用 (Dangling References)

* 在具有指標的語言中，很容易通過釋放內存時，保留原本指向他的指標而產生錯誤的垂懸指標。
* 垂懸指標 (dangling pointer)：指向的內存已經被非配給其他擁有者。

```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```
會產生編譯錯誤，因為`s`是在函數內創建的，當該函數執行完畢後，`s`會被釋放，如果嘗試傳回他的引用值，表示這個引用會指向一個無效的`String`結構。

解決方法是直接傳回`String`：
```
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
