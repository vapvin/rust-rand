struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    println!(
        "사각형의 면적: {}제곱픽셀",
        area(&rect1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectamgle.width * rectangle.height
}