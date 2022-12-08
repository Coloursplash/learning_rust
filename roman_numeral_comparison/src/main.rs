// https://www.reddit.com/r/dailyprogrammer/comments/oe9qnb/20210705_challenge_397_easy_roman_numeral/
// Function to return whether the first roman numeral is smaller than the second

use std::collections::HashMap;

fn roman_compare(num1: &str, num2: &str) -> bool {
    let v1 = roman_to_num(num1);
    let v2 = roman_to_num(num2);

    return v1 < v2;
}

fn roman_to_num(num: &str) -> u32 {
    let vals = HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
        ]);

    let mut n = 0;

    for i in 0..(num.len() - 1) {
        let v1 = vals.get(&num.chars().nth(i).unwrap()).unwrap();
        let v2 = vals.get(&num.chars().nth(i + 1).unwrap()).unwrap();
        if v1 < v2 {
            n -= v1;
        } else {
            n += v1;
        }
    }

    return n;
}

fn main() {
    let num1 = "MDCLXV";
    let num2 = "MDCLXVI";

    let result = roman_compare(num1, num2);

    println!("{}", result);
}
