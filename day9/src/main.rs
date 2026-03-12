fn analyze_game_score(score: i32) -> (char, bool, i32) {
    // 實作這裡
    let diff = 100 - score;
    let passed = score >= 60;
    let mut rank = 'F';

    if score >= 90 {
        rank = 'S';
    } else if score >= 70 {
        rank = 'A';
    } else if score >= 50 {
        rank = 'B';
    }

    (rank, passed, diff)
}

fn main() {
    let name = "お兄ちゃん";
    let (rank, passed, diff) = analyze_game_score(85);
    // 用解構取得三個值
    // 印出結果
    println!("Name: {}", name);
    println!("Rank: {} | Passed: {} | Diff: {}", rank, passed, diff);
}
