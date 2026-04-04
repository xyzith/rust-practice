use std::fmt::Display;

// TODO: 定義 Validator 結構
// 提示：需要兩個欄位 min 和 max，類型是 T
struct Validator<T> {
    // 填入欄位
    max: T,
    min: T,
}

impl<T> Validator<T>
where
    T: PartialOrd + Display,
{
    // TODO: 實作 new() 方法
    fn new(min: T, max: T) -> Self {
        // 填入實作
        Self { min, max }
    }

    // TODO: 實作 validate() 方法
    // 檢查 value 是否在 min 和 max 之間
    // 如果在範圍內，印出 "✅ {value} is valid"
    // 如果不在，印出 "❌ {value} is out of range"
    fn validate(&self, value: &T) {
        // 填入實作
        if (self.min <= *value) && (*value <= self.max) {
            // Q1: 這裡為什麼要用 *value 而不是 value？(這個問題我可能會問很多次直到我弄懂)
            // Q2: 為何大小運算服能用但 min..max 不能用
            println!("✅ {} is valid", value);
        } else {
            println!("❌ {} is out of range", value);
        }

        // if (self.min..self.max).contains(value) {
        // Q: 我要怎麼向rust保證T是合法的
    }
}

fn main() {
    // 測試整數 (1-100)
    let int_validator = Validator::new(1, 100);
    int_validator.validate(&50);
    int_validator.validate(&150);

    // 測試浮點數 (0.0-1.0)
    let float_validator = Validator::new(0.0, 1.0);
    float_validator.validate(&0.5);
    float_validator.validate(&1.5);

    // 測試字串 ("A"-"Z")
    let str_validator = Validator::new("A", "Z");
    str_validator.validate(&"M");
    str_validator.validate(&"a");
}
