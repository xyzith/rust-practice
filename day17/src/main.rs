// 在陣列裡尋找特定元素，找到回傳 Some(index)，找不到回傳 None
fn find_item(items: &[&str], target: &str) -> Option<usize> {
    // TODO: 實作這個函數
    let idx = items.iter().position(|&item| item == target);
    match idx {
        Some(i) => Some(i),
        None => None,
    }
}

fn main() {
    let fruits = ["apple", "banana", "cherry"];
    println!("{:?}", find_item(&fruits, "banana")); // Some(1)
    println!("{:?}", find_item(&fruits, "grape")); // None
    //
    let numbers = vec![10, 20, 30, 40, 50];

    // TODO: 用 match 處理 numbers.get(2)
    // - 如果有值，印出「索引 2 的值是：30」
    // - 如果沒有值，印出「索引超出範圍」
    match numbers.get(2) {
        Some(value) => println!("索引 2 的值是：{}", value),
        None => println!("索引超出範圍"),
    }

    // TODO: 用 unwrap_or 提供預設值 0
    let value = numbers.get(10).unwrap_or(&0);
    println!("取得的值：{}", value);
}
