fn describe_number(n: i32) -> String {
    if n > 0 {
        String::from("正數")
    } else if n < 0 {
        String::from("負數")
    } else {
        String::from("零")
    }
}

fn main() {
    // 整數
    let x: i32 = 42; // 32 位元有號整數（最常用）
    let y: u8 = 255; // 8 位元無號整數（0~255）

    // 浮點數
    let pi: f64 = 3.14; // 64 位元（預設）
    let e: f32 = 2.718; // 32 位元

    // 布林
    let is_cool: bool = true;

    // 字元（注意是單引號，可以是 emoji！）
    let heart: char = '💖';

    println!(
        "整數: {} {}, 浮點數: {} {}, 布林: {}, 字元: {}",
        x, y, e, pi, is_cool, heart
    );

    println!("{}", describe_number(5)); // 正數
    println!("{}", describe_number(-3)); // 負數
    println!("{}", describe_number(0)); // 零
}
