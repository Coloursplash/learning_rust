#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn diagonal(&self) -> f32 {
        let x = self.width as f32;
        let y = self.height as f32;
        (x * x + y * y).sqrt()
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 3,
        height: 4,
    };

    println!("{}, {}", rect1.diagonal(), rect1.area());

    let rect2 = Rectangle {
        width: 7,
        height: 12,
    };

    println!("{}, {}", rect2.diagonal(), rect2.area());
}
