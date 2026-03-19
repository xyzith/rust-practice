fn add_ten(num: &mut i32) {
    *num += 10;
}

struct BankAccount {
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        // TODO: 實作存款功能
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> bool {
        // TODO: 實作提款功能
        // 如果餘額不足回傳 false，否則扣款並回傳 true
        if self.balance >= amount {
            self.balance -= amount;
            true
        } else {
            false
        }
    }
}

fn double_values(numbers: &mut Vec<i32>) {
    // TODO: 將向量中的每個數字都乘以 2
    // 提示：使用 iter_mut()
    numbers.iter_mut().for_each(|x| *x *= 2);
}

fn main() {
    let mut value = 5;
    add_ten(&mut value);
    assert_eq!(value, 15);
    println!("Hello, world!");

    let mut account = BankAccount { balance: 100.0 };

    account.deposit(50.0);
    assert_eq!(account.balance, 150.0);
    assert!(account.withdraw(30.0));
    assert_eq!(account.balance, 120.0);
    assert!(!account.withdraw(200.0)); // 餘額不足

    let mut nums = vec![1, 2, 3, 4];
    double_values(&mut nums);
    assert_eq!(nums, vec![2, 4, 6, 8]);
}
