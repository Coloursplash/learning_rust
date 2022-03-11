use std::collections::HashMap;

fn main() {
    // Any variables inserted into the hashmap become invalid
    let mut scores = HashMap::new();

    scores.insert(String::from("James"), 100);
    scores.insert(String::from("Ruben"), 13);

    // Overwrites previous value for Ruben
    scores.insert(String::from("Ruben"), 64);

    let name = String::from("James");
    // .get() returns Option(&V)
    let score = scores.get(&name); // Reference the string
    match score {
        None => println!("{} does not have a score!", name),
        Some(num) => println!("{} scored {}%", name, num),
    }

    // This does it in an arbitrary order
    for (key, value) in &scores {
        println!("{}: {}%", key, value);
    }

    // Only insert if the key has no value
    scores.entry(String::from("Harry")).or_insert(57);
    scores.entry(String::from("James")).or_insert(86);


    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
