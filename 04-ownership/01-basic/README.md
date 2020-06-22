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