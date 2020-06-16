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

解決：把 `C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Tools\MSVC\14.25.28610\bin\Hostx64\x64` 加入環境變數

參考 -- https://github.com/rust-lang/rustup/issues/1455