// 回傳較長的字串切片
fn get_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    // 你的程式碼
    return if s1.len() > s2.len() { s1 } else { s2 };
}

// 回傳字串的第一個字（如果存在）
// 提示：使用 split_whitespace() 和 next()
fn first_word<'a>(s: &'a str) -> Option<&'a str> {
    // 你的程式碼
    return s.split_whitespace().next();
}

// 一個持有字串引用的結構體
struct Highlight<'a> {
    text: &'a str,
}

// 實作一個方法回傳文字長度
impl<'a> Highlight<'a> {
    fn length(&self) -> usize {
        // 你的程式碼
        return self.text.len();
    }
}

fn main() {
    // 測試 1: get_longer
    let s1 = "hello";
    let s2 = "world!";
    println!("較長的是: {}", get_longer(s1, s2)); // 應該印出 "world!"

    // 測試 2: first_word
    let sentence = "Rust is awesome";
    match first_word(sentence) {
        Some(word) => println!("第一個字: {}", word), // 應該印出 "Rust"
        None => println!("空字串"),
    }

    // 測試 3: Highlight
    let text = "Important!";
    let highlight = Highlight { text };
    println!("長度: {}", highlight.length()); // 應該印出 10
}
