// 1. 定義 trait
trait Describable {
    fn describe(&self) -> String;

    fn short_desc(&self) -> String {
        String::from("A thing")
    }
}

// 2. 定義結構體
struct Person {
    // TODO
    name: String,
    age: u32,
}

struct Product {
    // TODO
    name: String,
    price: f64,
}

// 3. 實作 trait
impl Describable for Person {
    // TODO
    fn describe(&self) -> String {
        format!("{} is {} years old.", self.name, self.age)
    }
    // Q: 假設我有一個fn想讓多個struct都能用，該怎麼寫，寫在外面不讓我用self當參數

    fn short_desc(&self) -> String {
        format!("{} is a person.", self.name)
    }
}

impl Describable for Product {
    // TODO
    fn describe(&self) -> String {
        format!("{} costs ${:.2}.", self.name, self.price)
    }
    fn short_desc(&self) -> String {
        format!("{} is a product.", self.name)
    }
}

fn main() {
    let person = Person {
        name: String::from("Alice"),
        // Q: 關於function的呼叫方式，用.和::有什麼不同？類似lua一個自帶slf參數一個不帶的差別嗎
        age: 30,
    };

    let product = Product {
        name: String::from("Laptop"),
        price: 999.99,
    };

    // 測試你的實作
    println!("{}", person.describe());
    println!("{}", person.short_desc());
    println!("{}", product.describe());
    println!("{}", product.short_desc());
}
