# let
* 會發生錯誤
```
let x = 5;
x = 6;
```


* 加上 mut 使其可變
```
let mut x = 5
x = 6
```
# const
* 常數不可改變
```
const MAX_POINTS: u32 = 100_000;
```

# Shadowing
* 定義一個同名的變數
```
let x = 5;

let x = x + 1;

let x = x * 2;

println!("The value of x is: {}", x);
```
* 比較 : `mut`改變值，而 Shadow 創建一個新的變數

* 型態錯誤：字串和數字衝突
```
let mut spaces = "   ";
spaces = spaces.len();
```

