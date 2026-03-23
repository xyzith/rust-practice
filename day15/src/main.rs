fn sum_slice(slice: &[i32]) -> i32 {
    // 提示：可以用 for loop 或 iter().sum()
    slice.iter().sum()
}

fn first_word(s: &str) -> &str {
    // 提示：遍歷 bytes，找到第一個空格的位置
    // 然後回傳從開頭到該位置的切片
    let first_space_index = s.find(' ').unwrap_or(s.len());
    &s[..first_space_index]
}

fn main() {
    // 測試 first_word
    let sentence = String::from("Hello Rust World");
    println!("第一個字: {}", first_word(&sentence));

    // 測試 sum_slice
    let numbers = [10, 20, 30, 40, 50];
    println!("前三個數字的和: {}", sum_slice(&numbers[..3]));
    println!("全部數字的和: {}", sum_slice(&numbers));
}
