use std::io;

fn main() {
    let mut n: u32 = 0;
    let mut escape = false;

    while !escape {
        println!("Enter a number:");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        n = match input.trim().parse() {
            Ok(num) => {
                escape = true;
                num
            }
            Err(_) => continue,
        };
    }

    let mut cache = vec![0; (n as usize) + 1];
    cache[1] = 1;

    for i in 2..(n + 1) {
        cache[i as usize] = cache[(i - 1) as usize] + cache[(i - 2) as usize];
        println!("{} {}", i, cache[i as usize]);
    }

    println!("The {}th fibonacci number is {}", n, cache[n as usize]);
}
