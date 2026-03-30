fn print_if_some(opt: Option<i32>) {
    // TODO: 使用 if let 完成
    if let Some(value) = opt {
        println!("Value is: {}", value);
    }
}

fn describe_option(opt: Option<&str>) -> String {
    // TODO: 使用 if let ... else 完成
    if let Some(value) = opt {
        format!("Found: {}", value)
    } else {
        "Nothing found".to_string()
    }
}

fn print_reverse() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    // TODO: 使用 while let 完成
    while let Some(num) = numbers.pop() {
        println!("{} ", num);
    }
}

fn parse_number(s: &str) {
    let result = s.parse::<i32>();
    // TODO: 使用 if let 處理 Result
    if let Ok(num) = result {
        println!("Parsed: {}", num);
    } else {
        println!("Failed to parse");
    }
}

fn main() {
    // 測試
    print_if_some(Some(42)); // 應該印出: Value is: 42
    print_if_some(None); // 不印出任何東西
    //
    // 測試
    assert_eq!(describe_option(Some("treasure")), "Found: treasure");
    assert_eq!(describe_option(None), "Nothing found");

    print_reverse(); // 應該印出: 5 4 3 2 1

    // 測試
    parse_number("123"); // 應該印出: Parsed: 123
    parse_number("abc"); // 應該印出: Failed to parse
}
