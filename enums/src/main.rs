enum Packet {
    Move {x: i32, y: i32, z: i32},
    Color(u8, u8, u8),
    Heal(f32),
    UpdateName(String),
}

impl Packet {
    fn call(&self) {
        // method body would be defined here
    }
}

enum Coin {
    Galleon,
    Sickle,
    Knut
}

fn main() {
    let example_packet = Packet::Move {x: 3, y: 5, z: 1};
    example_packet.call();

    let galleon = Coin::Galleon;
    println!("A galleon is worth £{}.", value_in_pounds(&galleon));

    if let Coin::Galleon = galleon {
        println!("This coin is a galleon.");
    }

    let dice_roll = 5;
    match dice_roll {
        3 => add_fancy_hat(),
        5 => remove_fancy_hat(),
        _ => reroll(),
    }

    let dice_roll = 12;
    match dice_roll {
        2 => add_fancy_hat(),
        other => move_spaces(other), // Allows us to pass value to function
    }
}

fn value_in_pounds(coin: &Coin) -> f32 {
    match coin {
        Coin::Galleon => 4.93,
        Coin::Sickle => 0.29,
        Coin::Knut => 0.01,
    }
}

fn add_fancy_hat() {}

fn remove_fancy_hat() {}

fn reroll() {}

fn move_spaces(_spaces: u32) {}
