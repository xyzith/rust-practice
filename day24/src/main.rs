fn gen_get_high_low(threshold: i32) -> impl Fn(i32) -> bool {
    // Q: 這種fn該怎麼命名比較好
    let _threshold = threshold;
    // Q: 需要另外存嗎？還是可以用'保障生命週期？
    move |x: i32| x > _threshold
    // Q1: move 关键字的作用是什么？为什么需要它？
    // Q2: |x: i32| 這段是什麼意思，為何和fn 宣告長得不一樣
}

fn gen_incrementer() -> impl FnMut() -> i32 {
    let mut count = 0;
    move || {
        // Q: || 又是啥
        count += 1;
        count
    }
}
fn gen_cusume() -> impl FnOnce() -> usize {
    let data = vec![1, 2, 3, 4, 5];
    move || {
        let len = data.len();
        drop(data); // 顯式釋放所有權
        len
    }
}

fn gen_filter_vec(vec: Vec<i32>) -> impl for<'a> Fn(&'a (dyn Fn(&i32) -> bool + 'a)) -> Vec<i32> {
    move |filter: &dyn Fn(&i32) -> bool| vec.iter().filter(|x| filter(*x)).copied().collect()
}

fn main() {
    let is_large = gen_get_high_low(10);
    println!("{}", is_large(15)); // true
    println!("{}", is_large(5)); // false
    //
    let mut increment = gen_incrementer();
    // Q: 為何fu也要mut
    println!("{}", increment()); // 1
    println!("{}", increment()); // 2
    //
    let consume = gen_cusume();
    println!("{}", consume()); // 5
    // consume(); // ❌ 錯誤：只能呼叫一次

    let numbers = vec![1, 2, 3, 4, 5, 6];
    let filter_vec = gen_filter_vec(numbers);
    let filter_evens = |x: &i32| x % 2 == 0;
    println!("{:?}", filter_vec(&filter_evens)); // [2, 4, 6]
}
