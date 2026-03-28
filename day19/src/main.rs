enum TrafficLight {
    Red,
    Yellow,
    Green,
}

// TODO 1: 寫一個函數 light_action，接收 TrafficLight，回傳對應動作
// Red => "Stop"
// Yellow => "Slow down"
// Green => "Go"
fn light_action(light: TrafficLight) -> &'static str {
    // 在這裡使用 match
    match light {
        TrafficLight::Red => "Stop",
        TrafficLight::Yellow => "Slow down",
        TrafficLight::Green => "Go",
    }
}

// TODO 2: 寫一個函數 classify_number，接收 i32，回傳分類
// 負數 => "negative"
// 0 => "zero"
// 1..=10 => "small positive"
// 11..=100 => "medium positive"
// 其他 => "large positive"
fn classify_number(n: i32) -> &'static str {
    // 在這裡使用 match
    match n {
        x if x < 0 => "negative",
        0 => "zero",
        1..=10 => "small positive",
        11..=100 => "medium positive",
        _ => "large positive",
    }
}

// TODO 3: 寫一個函數處理 Result
// 如果是 Ok(value)，回傳 value * 2
// 如果是 Err(_)，回傳 0
fn double_or_zero(result: Result<i32, String>) -> i32 {
    // 在這裡使用 match
    match result {
        Ok(value) => value * 2,
        Err(_) => 0,
    }
}

fn main() {
    // 測試 TODO 1
    println!("Red light: {}", light_action(TrafficLight::Red));
    println!("Green light: {}", light_action(TrafficLight::Green));
    // 原本沒用到所以補一下
    println!("Green light: {}", light_action(TrafficLight::Yellow));

    // 測試 TODO 2
    println!("-5: {}", classify_number(-5));
    println!("7: {}", classify_number(7));
    println!("50: {}", classify_number(50));
    println!("200: {}", classify_number(200));

    // 測試 TODO 3
    println!("Ok(10): {}", double_or_zero(Ok(10)));
    println!("Err: {}", double_or_zero(Err("error".to_string())));
}
