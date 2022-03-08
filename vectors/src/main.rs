enum SpreadsheetCell {
    Int(i64),
    Float(f64),
    Text(String),
}

fn main() {
    let mut v: Vec<i32> = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    //&v[100] // This would crash
    v.get(100); // Will return None

    for i in &v {
        println!("{}", i);
    }

    let mut x = vec![100, 42, 21];
    for i in &mut x {
        *i += 50; // * dereferences i
    }
    println!("x");
    for i in x {
        println!("{}", i);
    }

    // Using an enum allows for storing multiple types
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(12.15),
        SpreadsheetCell::Text(String::from("The grand old duke of York")),
    ];
}
