fn swap<T, U>(pair: (T, U)) -> (U, T) {
    let (a, b) = pair;
    (b, a)
    // Q: 這邊IDE原本想讓我用.swap() 但型別我怎麼弄都不對。
}

struct Container<T> {
    value: T,
}

impl<T> Container<T> {
    fn new(value: T) -> Self {
        // 你的程式碼
        Self { value }
        // Q: 這是類似 constructor？我不確定這樣寫對不對
    }

    fn get(&self) -> &T {
        // 你的程式碼
        &self.value
    }
}

struct Wrapper<T, U> {
    inner: T,
    metadata: U,
}

impl<T, U> Wrapper<T, U> {
    fn wrap(value: T, meta: U) -> Self {
        // 你的程式碼
        Self {
            inner: value,
            metadata: meta,
        }
    }

    fn unwrap(self) -> (T, U) {
        // 你的程式碼
        (self.inner, self.metadata)
        // Q: 有沒有更好的寫法，strust怎麼解構
    }
}

fn main() {
    // 測試
    let pair = ("hello", 42);
    let swapped = swap(pair);
    println!("{:?}", swapped); // (42, "hello")

    // 測試
    let num_container = Container::new(42);
    let str_container = Container::new("Rust");
    println!("{}", num_container.get()); // 42
    println!("{}", str_container.get()); // Rust
    //
    // 測試
    let wrapped = Wrapper::wrap(vec![1, 2, 3], "numbers");
    let (data, label) = wrapped.unwrap();
    println!("{:?} - {}", data, label); // [1, 2, 3] - numbers
}
