fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);

        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 8 {
                break;
            }
            if count == 2 {
                break 'counting_up
            }
            remaining -= 1;
        }

        count += 1;
    }

    
    // RETURNING FROM LOOP
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    // CONDITIONAL LOOPS
    let mut n = 5;
    while n != 0 {
        println("{}!", n);

        n -= 1;
    }

    println!("LIFT OFF!");

    // Alternative with for loop
    for n in (1..4).rev() {
        println!("{}!", n);
    }
    println!("LIFT OFF!");

    // LOOPING THROUGH ARRAY
    let arr = [1, 4, 9, 16, 25, 36, 49, 64, 72, 81, 100];
    let mut idx = 0;

    while idx < arr.len() {
        println!("the value of index {} is {}", idx, arr[idx]);
        idx += 1;
    }

    // Alternative with for loop
    for element in arr {
        println("the value is {}", element);
    }
}
