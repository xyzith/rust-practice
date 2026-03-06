// Array 版本：固定大小，回傳 Vec
fn filter_even_array(arr: [i32; 5]) -> Vec<i32> {
    arr.iter().copied().filter(|&x| x % 2 == 0).collect()
}

// Vec 版本：動態大小，修改原地或回傳新的
fn filter_even_vec(vec: &Vec<i32>) -> Vec<i32> {
    return vec.iter().filter(|&&x| x % 2 == 0).copied().collect();
}

fn sum_array(arr: [i32; 5]) -> i32 {
    return arr.iter().sum();
    // let mut sum = 0;
    // for n in arr {
    //     sum += n;
    // }
    // return sum;
}

fn sum_vec(vec: Vec<i32>) -> i32 {
    return vec.iter().sum();
    // let mut sum = 0;
    // for n in vec {
    //     sum += n;
    // }
    // return sum;
}

fn main() {
    let arr = [1, 2, 3, 4, 5];
    println!("Array sum: {}", sum_array(arr));

    let vec = vec![10, 20, 30];
    println!("Vec sum: {}", sum_vec(vec));

    let arr = [1, 2, 3, 4, 5];
    println!("Array evens: {:?} orig: {:?}", filter_even_array(arr), arr);

    let vec = vec![10, 15, 20, 25, 30];
    println!("Vec evens: {:?} orig: {:?}", filter_even_vec(&vec), vec);
}
