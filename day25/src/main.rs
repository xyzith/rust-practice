fn process_numbers(nums: Vec<i32>) -> Vec<i32> {
    // 1. 篩選出偶數
    // 2. 每個數字乘以 3
    // 3. 收集成 Vec 回傳
    nums.into_iter()
        .filter(|x| x % 2 == 0)
        .map(|x| x * 3)
        .collect()
}

fn count_long_words(words: Vec<&str>) -> usize {
    // 計算長度 > 5 的單字數量
    // 提示: 使用 filter 和 count()
    words.into_iter().filter(|w| w.len() > 5).count()
}

fn sum_of_squares(n: i32) -> i32 {
    // 計算 1² + 2² + ... + n²
    // 提示: 使用範圍 (1..=n) 和 map, sum
    // let v: Vec<i32> = vec![1; n + 1];
    // v.into_iter().enumerate().map(|x| x * x).sum()
    // Q: 這邊型別轉換搞死我心態了，usize到底是什麼狗屎格式不能加不能乘?
    let v: Vec<i32> = (1..n).collect();
    v.into_iter().map(|x| (x * x) as i32).sum()
}
fn top_three_doubled(nums: Vec<i32>) -> Vec<i32> {
    // 1. 排序（大到小）
    // 2. 取前 3 個
    // 3. 每個乘以 2
    // 提示: 需要先 sort，然後用 iter() 處理
    let mut nums = nums;
    nums.sort();
    nums.reverse();
    // Q:
    // 邏輯我懂，但是會死在型態轉換和語法，麻煩以後不要再出這種提示不足的題目了，我現在的問題不是沒程式邏輯，而是Rust不熟。
    nums.into_iter().take(3).map(|x| x * 2).collect()
    // Q: 結果對了但是我無法chain 起來，
}

fn main() {
    let cal1 = process_numbers(vec![1, 2, 3, 4, 5, 6]);
    // 應該回傳: vec![6, 12, 18]
    // Q: {:?} 代表什麼意思
    println!("{:?}", cal1);

    let cal2 = count_long_words(vec!["hello", "world", "rust", "programming"]);
    // 應該回傳: 2 ("programming" 長度 11)
    println!("{}", cal2);

    let cal3 = sum_of_squares(5);
    // 應該回傳: 55  (1+4+9+16+25)
    println!("{}", cal3);

    let cal4 = top_three_doubled(vec![3, 1, 4, 1, 5, 9, 2]);
    // 應該回傳: vec![18, 10, 8]  (9*2, 5*2, 4*2)
    println!("{:?}", cal4);
}
