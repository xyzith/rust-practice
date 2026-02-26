// 請實作這個函數
fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64 {
    // 你的程式碼
    return weight_kg / (height_m * height_m);
    // return weight_kg / (height_m.powf(2.0));
}

fn main() {
    let bmi = calculate_bmi(106.0, 1.73);
    println!("BMI: {:.2}", bmi);
}
