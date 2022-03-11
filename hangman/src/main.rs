use std::fs;
use rand::Rng;
use std::io::stdin;

fn open_file(file_name: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
        .expect("Something went wrong reading the file");

    let mut words: Vec<String> = Vec::new();
    let mut prev_idx = 0;
    for (i, &item) in contents.as_bytes().iter().enumerate() {
        if item == b',' {
            words.push(String::from(&contents[prev_idx..i]));
            prev_idx = i + 1;
        }
    }
    words.push(String::from(&contents[prev_idx..]));
    words
}

fn get_random_word(words: Vec<String>) -> String {
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(0..words.len());
    String::from(&words[n as usize])
}

fn main() {
    let file_name = "words.csv";
    let words = open_file(&file_name);
    let word = get_random_word(words);

    let mut guessed_letters: Vec<char> = Vec::new();
    let mut guesses:u8 = 6;
    let mut exit = false;

    while guesses > 0 && !exit {
        println!("{} guesses left", guesses);
        for c in word.chars() {
            if guessed_letters.contains(&c) {
                print!("{}", c);
            } else {
                print!("_");
            }
        }
        println!();
        println!("Enter a letter:");
    
        let mut temp_inp = String::new();
        stdin()
            .read_line(&mut temp_inp)
            .expect("Unable to read line");
    
        let guessed_char: char = match temp_inp.trim().parse() {
            Ok(c) => c,
            Err(_) => continue,
        };

        guessed_letters.push(guessed_char);
    }
}
