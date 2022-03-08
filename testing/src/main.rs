fn main() {
    let mut x = String::from("Hello, ");

    x.push('w');
    x.push_str("orld!");
    
    println!("{}", x);
}
