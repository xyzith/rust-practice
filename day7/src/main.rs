fn main() {
    let name = "Lain";
    println!("{}", greet(&name));
    println!("test var name: {}", name);
}

fn greet(name: &str) -> String {
    let r = format!("Hello, {}!", name);
    return r;
}
