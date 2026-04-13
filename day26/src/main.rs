// 在同一個檔案中定義模組
mod math_utils {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    pub fn multiply(a: i32, b: i32) -> i32 {
        a * b
    }

    fn secret_formula(x: i32) -> i32 {
        x * 2 + 10
    }
    // Q: 這沒用到會有warning，但有時候就是備而不用時怎處理？
}

mod string_utils;
// Q: 要我練習improt模駔也請給我語法提示，不要每次讓我通靈。

fn main() {
    use math_utils::{add, multiply};
    println!("{}", add(2, 3));
    println!("{}", multiply(2, 3));

    // use math_utils::secret_formula;

    use string_utils::{to_lowercase, to_uppercase};
    println!("{}", to_uppercase("hello"));
    println!("{}", to_lowercase("WORLD"));
}
