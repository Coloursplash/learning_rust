fn main() {
    let s1 = String::from("Hello and welcome to another Minecraft Let's Play video.");
    let s2 = "This is a string literal";

    println!("{} {}", first_word(&s1), first_word(&s2));

    let s3 = String::from("Hello world!");
    let s3_slice = &s3[6..]; // Must be reference to string
    println!("{}", s3_slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
