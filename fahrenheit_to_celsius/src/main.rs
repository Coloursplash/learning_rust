use std::io;

fn main() {
    let mut fahrenheight: i32 = 0;
    let mut escape = false;
    
    while !escape {
        println!("Enter a number: ");
        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        fahrenheight = match guess.trim().parse() {
            Ok(num) => {
                escape = true;
                num
            },
            Err(_) => continue,
        };
    }

    let celsius = (fahrenheight as f32 - 32.0) / 1.8;
    println!("{} F in celsius is {} C.", fahrenheight, celsius);
}
