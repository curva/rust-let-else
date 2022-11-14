#[derive(Clone, Copy)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let point = Point { x: 10, y: 20 };

    let Point {x, y} = point;
    println!("x = {}, y = {}", x, y);

    if let Point { x: 0, y } = point {
        println!("x = {}, y = {}", x, y);
    }

    if let Point { x: 10, y } = point {
        println!("x = {}, y = {}", x, y);
    }
}
