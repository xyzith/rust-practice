fn parse_number(s: &str) -> Result<i32, String> {
    // 提示：使用 s.parse::()
    // 它回傳 Result
    // 用 .map_err() 轉換錯誤型別
    s.parse::<i32>().map_err(|e| e.to_string())
}

fn safe_calculator(a_str: &str, b_str: &str) -> Result<i32, String> {
    // 提示：使用 ? 運算子串接多個 Result
    // 1. parse_number(a_str)?
    // 2. parse_number(b_str)?
    // 3. 檢查除數是否為 0
    let a = parse_number(a_str)?;
    let b = parse_number(b_str)?;
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    match parse_number("42") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error parsing number: {}", e),
    }
    match parse_number("abc") {
        Ok(n) => println!("Parsed number: {}", n),
        Err(e) => println!("Error parsing number: {}", e),
    }

    // 1. 正常計算：12 / 3
    // 2. 除以零：12 / 0
    // 3. 無效輸入："abc" / "5"

    // 使用 match 或 if let 處理結果

    match safe_calculator("12", "3") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match safe_calculator("12", "0") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
    match safe_calculator("abc", "5") {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
