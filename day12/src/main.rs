fn get_length(s: &String) -> usize {
    // 回傳字串的長度
    s.len()
}

fn find_max(numbers: &Vec<i32>) -> Option<&i32> {
    // 提示：用 iter() 和 max()
    // 或者用 for 迴圈手動找
    numbers.iter().max()
}

struct User {
    name: String,
    age: u32,
}

impl User {
    fn print_user_info(&self) {
        println!("Name: {}, Age: {}", self.name, self.age);
    }
}

fn main() {
    let text = String::from("Rust");
    assert_eq!(get_length(&text), 4);
    println!("text 仍然可用: {}", text);

    let nums = vec![3, 7, 2, 9, 4];
    assert_eq!(find_max(&nums), Some(9).as_ref());

    let user = User {
        name: String::from("Alice"),
        age: 30,
    };

    user.print_user_info();
}
