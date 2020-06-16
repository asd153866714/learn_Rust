# Linux 或 macOS 安裝

`$ curl https://sh.rustup.rs -sSf | sh`

記得加入 PATH

# Windows 安裝

https://www.rust-lang.org/install.html

# 編譯並執行
```
user@DESKTOP-9VVBDPS MINGW64 /d/Xing/learn_Rust/00-bsic (master)
$ rustc main.rs

user@DESKTOP-9VVBDPS MINGW64 /d/Xing/learn_Rust/00-bsic (master)
$ ./main
Hello world!  
```

### 問題
```
rustc main.rs

error: linking with ` link.exe` failed: exit code: 1
```

解決：把 `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC\14.25.28610\bin\Hostx64\x64` 加入環境變數 PATH

參考 -- https://github.com/rust-lang/rustup/issues/1455

# 使用 Cargo
### 建立專案
`cargo new <專案名稱>`

* 可以被用來相依進其它專案中作為函式庫使用的函式庫程式專案
```
cargo new --lib <專案名稱>
```
### 編譯
```
cargo build
```
### 執行
```
cargo run
```
沒有事先進行編譯的話，直接使用 cargo run 也會先進行編譯

### 發布程式
```
cargo build --release
```

### 測試
```
cargo test
```

### 刪除
把整個 target 目錄刪除
```
cargo clean
```
# rustfmt 格式化
```
rustfmt main.rs

// 格式化 Cargo 專案
cargo fmt
```
