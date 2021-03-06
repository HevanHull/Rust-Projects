#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area (&self) -> u32{
        self.width * self.height
    }
}

fn main() {
    let q = Rectangle{
        width: 40, 
        height: 30
    };

    println!("area of the rectangle is: {}", q.area());
}
