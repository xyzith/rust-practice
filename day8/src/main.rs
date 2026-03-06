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
}
