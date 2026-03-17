fn main() {
    // 練習 1: 觀察所有權轉移
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有權轉移給 s2

    // println!("{}", s1);  // 取消註解會編譯失敗
    println!("s2 = {}", s2);

    // 練習 2: 函數與所有權
    // 完成 takes_ownership 函數
    let message = String::from("你好");
    takes_ownership(message);
    // println!("{}", message); // 取消註解會編譯失敗

    // 練習 3: 回傳所有權
    let s3 = gives_ownership();
    println!("got: {}", s3);

    // 練習 4: 讓這段程式碼可以編譯
    // 提示：使用 .clone() 複製資料
    let original = String::from("Rust");
    let copy = /* 在這裡修改 */ original.clone();
    println!("original: {}, copy: {}", original, copy);
}

// 完成這個函數：接收一個 String 並印出
fn takes_ownership(s: String) {
    println!("{}", s);
}

// 完成這個函數：建立並回傳一個 String
fn gives_ownership() -> String {
    return String::from("Hello from gives_ownership!");
}
