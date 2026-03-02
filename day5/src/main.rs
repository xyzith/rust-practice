// 1. 產生一個 1-10 的隨機數字（先用固定值 7 測試）
// 2. 讓使用者輸入猜測（可以用 hardcode 測試）
// 3. 用 loop 和 if/else 檢查：
//    - 太大 → 印 "Too high!"
//    - 太小 → 印 "Too low!"
//    - 正確 → 印 "Correct!" 然後 break
use rand::Rng;

fn main() {
    let secret = rand::thread_rng().gen_range(1..=10);
    let guesses = vec![5, 9, 7, 1]; // 模擬使用者輸入
    let mut guesses2 = vec![5, 9, 7, 1];
    // 你的 loop 寫在這裡
    for guess in guesses {
        if guess < secret {
            println!("Too low!");
        } else if guess > secret {
            println!("Too high!");
        } else {
            println!("Correct!");
            break;
        }
    }

    guesses2.reverse();
    loop {
        let n = guesses2.pop();
        match n {
            Some(v) => {
                if v > secret {
                    println!("Too high");
                } else if v < secret {
                    println!("Too Low");
                } else {
                    println!("Correct!");
                    break;
                }
            }
            None => break,
        }
    }

    println!("Secret is: {}", secret);
}
