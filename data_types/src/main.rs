fn main() {
    // Numbers can defined using decimal, hex, octal or binary
    // notion. An underscore can be used for formatting

    // Int types are either signed (i) or unsigned(u)
    // u8, u16, u32, u64
    // i8, i16, i32, i64
    let int = 3; // i64 by default
    let hex_int = 0xff;
    let oct_int = 0o77;
    let bin_int = 0b1111_0000;

    // Float types
    // f32, f64
    let float: f32 = 2.0;
    let long_float: f64 = 67.12934;

    // Boolean
    let boolean = true;
    let alt_bool = false;

    // Char characters use single quotes
    let a = 'a';
    let heart_eyed_cat = '😻';

    // Compound groups
    
    // Tuple
    // Can declare types or can be inferred
    let tup: (i32, f64, u8) = (500, 6.4, 255);
    let (tup_x, tup_y, tup_z) = tup;
    let alt_tup_x = tup.0;

    // Array
    // All items must have the same type
    let arr = [1, 2, 3, 4, 5];
    let three_arr = [3; 5]; // = [3, 3, 3, 3, 3]
    let alt_arr: [i32; 5] = [-15, 252, -195, 593031, 5];
    let arr_x = arr[0];

    // String
    let mut hello_world = String::new();
    hello_world.push('H'); // Pushes char
    hello_world.push_str("ello, world!"); // Pushes string
    println!("{}", hello_world);

    let mut hello = String::from("Dobrý");
    let den = "den";
    hello.push(' ');
    // push_str doesn't take ownership of den so can sti be used later
    hello.push_str(den);
    println!("{}", hello);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // Must use String + reference to String here
    let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
    println!("{}", s3);

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // Both of these doe the same thing
    let tictactoe = format!("{}-{}-{}", tic, tac, toe);
    let tictactoe = tic + "-" + &tac + "-" + &toe; // This borrows tic, making it unusable
    println!("{}", tictactoe);
    let tac = &tictactoe[..3]; // Create a slice from String
    println!("{}", tac);

    // This shows how unicode strings are saved and interpretted
    let hindi = "“नमस्ते”";
    let hindi_bytes = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    224, 165, 135];
    let hindi_unicode_scalar_vals = ['न', 'म', 'स', '्', 'त', 'े'];
    let hindi_unicode_grapheme_clusters = ["न", "म", "स्", "ते"];
    //let s = &hindi[..3] // This would cause a crash due to the special unicode byte array
    let hindi_bytes = hindi.bytes();
    let hindi_unicode_scalar_vals = hindi.chars();
}
