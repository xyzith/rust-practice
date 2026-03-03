fn main() {
    let name = String::from("Taylor");

    // 1. 把 name 傳給這個函數
    greet(&name);

    // 2. 試著再印一次 name，看看會發生什麼
    println!("Name again: {}", name);

    // 3. 修正方式：改用 &name（借用）
}

fn greet(s: &String) {
    println!("Hello, {}!", s);
}
